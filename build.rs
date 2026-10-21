fn main() {
    // winresource drives rc.exe (MSVC) / windres (GNU); the target-OS gate
    // keeps non-Windows hosts free of that toolchain.
    if std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows") {
        let mut res = winresource::WindowsResource::new();
        let icon = "crates/corpora_shell/src/shell/assets/app-icon/corpora.ico";
        res.set_icon(icon);
        res.compile().expect("embed Corpora app icon");
        println!("cargo:rerun-if-changed={icon}");
    }
}
