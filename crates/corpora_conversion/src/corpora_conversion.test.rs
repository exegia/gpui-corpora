#![cfg(unix)]

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

use super::{EpubTools, platform_key};
use std::ffi::OsStr;

fn executable(path: &Path) {
    fs::write(path, "#!/bin/sh\nexit 0\n").expect("write executable fixture");
    let mut permissions = fs::metadata(path)
        .expect("read fixture metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("make fixture executable");
}

fn app_paths(root: &Path) -> (PathBuf, PathBuf) {
    if std::env::consts::OS == "macos" {
        let contents = root.join("Corpora.app/Contents");
        (
            contents.join("MacOS/Corpora"),
            contents
                .join("Resources/corpora-tools")
                .join(platform_key()),
        )
    } else {
        let executable = root.join("bin/Corpora");
        let runtime = root.join("bin/corpora-tools").join(platform_key());
        (executable, runtime)
    }
}

#[test]
fn discovers_tools_from_the_app_bundle() {
    let temporary = tempfile::tempdir().expect("create fixture directory");
    let (executable_path, runtime) = app_paths(temporary.path());
    fs::create_dir_all(executable_path.parent().expect("executable parent"))
        .expect("create app executable directory");
    fs::create_dir_all(runtime.join("epub-to-tf")).expect("create exporter directory");
    fs::create_dir_all(runtime.join("python/bin")).expect("create Python runtime directory");
    executable(&executable_path);
    executable(&runtime.join("corpora-epub-check"));
    executable(&runtime.join("epub-to-tf/epub-to-tf"));
    executable(&runtime.join("python/bin/python3.13"));
    fs::write(runtime.join("corpora-sidecar.py"), "# fixture\n").expect("write sidecar entry");

    let tools = EpubTools::discover_for_executable(&executable_path).expect("discover tools");

    assert_eq!(tools.checker, runtime.join("corpora-epub-check"));
    assert_eq!(tools.exporter, runtime.join("epub-to-tf/epub-to-tf"));
    assert_eq!(tools.python, runtime.join("python/bin/python3.13"));
}

#[test]
fn configures_sidecar_with_bundled_absolute_paths() {
    let temporary = tempfile::tempdir().expect("create fixture directory");
    let tools = EpubTools {
        checker: temporary.path().join("checker"),
        exporter: temporary.path().join("exporter"),
        python: temporary.path().join("python"),
        sidecar_entry: temporary.path().join("sidecar.py"),
    };
    let command = tools.sidecar_command();

    let environment = command.get_envs().collect::<Vec<_>>();
    assert!(environment.iter().any(|(key, value)| {
        *key == OsStr::new("CORPORA_EPUB_CHECK_BIN") && *value == Some(tools.checker.as_os_str())
    }));
    assert!(environment.iter().any(|(key, value)| {
        *key == OsStr::new("CORPORA_EPUB_TO_TF_BIN") && *value == Some(tools.exporter.as_os_str())
    }));
    assert!(environment.iter().any(|(key, value)| {
        *key == OsStr::new("PYTHONDONTWRITEBYTECODE") && *value == Some(OsStr::new("1"))
    }));
    assert_eq!(command.get_program(), tools.python.as_os_str());
    assert_eq!(
        command.get_args().collect::<Vec<_>>(),
        [
            tools.sidecar_entry.as_os_str(),
            OsStr::new("--protocol"),
            OsStr::new("1")
        ]
    );
}

#[test]
fn rejects_runtime_missing_one_of_the_tools() {
    let temporary = tempfile::tempdir().expect("create fixture directory");
    let (executable_path, runtime) = app_paths(temporary.path());
    fs::create_dir_all(executable_path.parent().expect("executable parent"))
        .expect("create app executable directory");
    fs::create_dir_all(runtime.join("epub-to-tf")).expect("create runtime directory");
    executable(&executable_path);
    executable(&runtime.join("corpora-epub-check"));

    assert!(EpubTools::discover_for_executable(&executable_path).is_err());
}
