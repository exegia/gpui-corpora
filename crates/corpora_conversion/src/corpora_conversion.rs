//! Locate the bundled EPUB tools and configure the local sidecar environment.

use std::{
    env,
    error::Error,
    fmt,
    path::{Path, PathBuf},
    process::Command,
};

const CHECKER_ENV: &str = "CORPORA_EPUB_CHECK_BIN";
const EXPORTER_ENV: &str = "CORPORA_EPUB_TO_TF_BIN";

/// The validator and exporter included in a desktop bundle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpubTools {
    /// Absolute path to `corpora-epub-check`.
    pub checker: PathBuf,
    /// Absolute path to the bundled `epub-to-tf` executable.
    pub exporter: PathBuf,
    /// Absolute path to the bundled CPython interpreter.
    pub python: PathBuf,
    /// Sidecar entry point that implements protocol version 1.
    pub sidecar_entry: PathBuf,
}

/// A failure to locate a supported, complete EPUB runtime in the app bundle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeUnavailable {
    searched: Vec<PathBuf>,
}

impl fmt::Display for RuntimeUnavailable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "bundled EPUB runtime unavailable; searched")?;
        for path in &self.searched {
            write!(formatter, " {}", path.display())?;
        }
        Ok(())
    }
}

impl Error for RuntimeUnavailable {}

impl EpubTools {
    /// Locate the runtime shipped beside the running app executable.
    pub fn discover() -> Result<Self, RuntimeUnavailable> {
        let executable = env::current_exe().unwrap_or_default();
        Self::discover_for_executable(&executable)
    }

    /// Locate tools relative to an app executable, including macOS app bundles.
    pub fn discover_for_executable(executable: &Path) -> Result<Self, RuntimeUnavailable> {
        let platform = platform_key();
        let roots = resource_roots(executable);
        let mut searched = Vec::with_capacity(roots.len());

        for root in roots {
            let runtime = root.join(&platform);
            searched.push(runtime.clone());
            if let Some(tools) = Self::from_runtime_dir(&runtime) {
                return Ok(tools);
            }
        }

        Err(RuntimeUnavailable { searched })
    }

    /// Set the tool paths expected by `corpora sidecar --protocol 1`.
    pub fn configure_sidecar(&self, command: &mut Command) {
        command
            .env(CHECKER_ENV, &self.checker)
            .env(EXPORTER_ENV, &self.exporter)
            .env("PYTHONNOUSERSITE", "1");
    }

    /// Create the bundled sidecar command without relying on a user Python installation.
    pub fn sidecar_command(&self) -> Command {
        let mut command = Command::new(&self.python);
        command.arg(&self.sidecar_entry).args(["--protocol", "1"]);
        self.configure_sidecar(&mut command);
        command
    }

    fn from_runtime_dir(directory: &Path) -> Option<Self> {
        let checker = directory.join("corpora-epub-check");
        let exporter = directory.join("epub-to-tf").join("epub-to-tf");
        let python = directory.join("python/bin/python3.13");
        let sidecar_entry = directory.join("corpora-sidecar.py");
        if is_executable(&checker)
            && is_executable(&exporter)
            && is_executable(&python)
            && sidecar_entry.is_file()
        {
            Some(Self {
                checker,
                exporter,
                python,
                sidecar_entry,
            })
        } else {
            None
        }
    }
}

fn platform_key() -> String {
    let os = match env::consts::OS {
        "macos" => "macos",
        "linux" => "linux",
        _ => return format!("{}-{}", env::consts::OS, env::consts::ARCH),
    };
    format!("{os}-{}", env::consts::ARCH)
}

fn resource_roots(executable: &Path) -> Vec<PathBuf> {
    let Some(directory) = executable.parent() else {
        return Vec::new();
    };

    if env::consts::OS == "macos" {
        return directory
            .parent()
            .map(|contents| vec![contents.join("Resources").join("corpora-tools")])
            .unwrap_or_default();
    }

    let mut roots = vec![directory.join("corpora-tools")];
    if let Some(parent) = directory.parent() {
        roots.push(parent.join("lib/gpui-starter/corpora-tools"));
    }
    if let Some(app_dir) = env::var_os("APPDIR") {
        roots.push(PathBuf::from(app_dir).join("usr/lib/gpui-starter/corpora-tools"));
    }
    roots
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        true
    }
}

#[cfg(test)]
#[path = "corpora_conversion.test.rs"]
mod tests;
