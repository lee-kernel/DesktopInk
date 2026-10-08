use std::{env, path::PathBuf, process::Command};
fn main() {
    println!("cargo:rerun-if-changed=assets/app.ico");
    println!("cargo:rerun-if-changed=assets/app.rc");
    if !env::var("CARGO_CFG_TARGET_OS").is_ok_and(|s| s == "windows") {
        return;
    }
    let arch = if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "aarch64" {
        "arm64"
    } else {
        "x64"
    };
    let rc = env::var_os("RC")
        .map(PathBuf::from)
        .or_else(|| {
            let root = PathBuf::from(
                env::var_os("ProgramFiles(x86)")
                    .unwrap_or_else(|| "C:\\Program Files (x86)".into()),
            )
            .join("Windows Kits/10/bin");
            let mut versions: Vec<_> = std::fs::read_dir(root)
                .ok()?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .collect();
            versions.sort();
            versions
                .into_iter()
                .rev()
                .map(|p| p.join(arch).join("rc.exe"))
                .find(|p| p.exists())
        })
        .unwrap_or_else(|| PathBuf::from("rc.exe"));
    let res = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("app.res");
    let status = Command::new(&rc)
        .arg("/nologo")
        .arg("/fo")
        .arg(&res)
        .arg("assets/app.rc")
        .status()
        .expect("Windows SDK rc.exe is required to embed the application icon");
    assert!(status.success(), "Resource compilation failed");
    println!("cargo:rustc-link-arg={}", res.display());
}
