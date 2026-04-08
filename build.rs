use chrono::Local;
use std::env::consts::{ARCH, OS};
use std::process::Command;

#[cfg(debug_assertions)]
const BUILD_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
const BUILD_TYPE: &str = "release";

fn get_commit_hash() -> String {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:%h") // Abbreviated commit hash
        // .arg("--pretty=format:%H") // Full commit hash
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        std::env::var("GITHUB_SHA")
            .ok()
            .map(|s| s[..7].to_string())
            .unwrap_or_else(|| "unknown".into())
    }
}

fn get_branch_name() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string()
    } else {
        std::env::var("GITHUB_REF_NAME").unwrap_or_else(|_| "unknown".into())
    }
}

fn is_working_tree_clean() -> bool {
    let status = Command::new("git")
        .arg("diff")
        .arg("--quiet")
        .arg("--exit-code")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();

    status.code().unwrap() == 0
}

fn main() {
    let use_rfd_gtk3 = std::env::var("CARGO_FEATURE_RFD_GTK3").is_ok();
    let use_rfd_xdg_portal = std::env::var("CARGO_FEATURE_RFD_XDG_PORTAL").is_ok();

    if use_rfd_gtk3 && use_rfd_xdg_portal {
        panic!("Features `rfd-gtk3` and `rfd-xdg-portal` are mutually exclusive.");
    }

    if !use_rfd_gtk3 && !use_rfd_xdg_portal {
        panic!("You must enable at least one of `rfd-gtk3` or `rfd-xdg-portal`.");
    }
    let rfd_backend = if use_rfd_xdg_portal {
        "xdg-portal"
    } else {
        "gtk3"
    };
    println!("cargo:rustc-env=RFD_BACKEND={}", rfd_backend);
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let branch_name = get_branch_name();
    let commit_hash = get_commit_hash();
    let commit_hash_plus = format!(
        "{}{}",
        commit_hash,
        if is_working_tree_clean() { "" } else { "+" }
    );
    let version_string = format!(
        "{} {} ({}-{}, {} build, {} [{}])",
        pkg_name, pkg_version, branch_name, commit_hash_plus, BUILD_TYPE, OS, ARCH
    );

    let short_version_string = format!("{}-{} ({})", pkg_version, commit_hash_plus, BUILD_TYPE);

    let build_date = Local::now().format("%d/%m/%Y").to_string();

    println!("cargo:rustc-env=BUILD_TYPE={}", BUILD_TYPE);
    println!("cargo:rustc-env=BRANCH_NAME={}", branch_name);
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
    println!("cargo:rustc-env=COMMIT_HASH_PLUS={}", commit_hash_plus);
    println!("cargo:rustc-env=VERSION_STRING={}", version_string);
    println!(
        "cargo:rustc-env=SHORT_VERSION_STRING={}",
        short_version_string
    );
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);

    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let host = std::env::var("HOST").unwrap(); // triple like x86_64-pc-windows-msvc

    let embed_icon = std::env::var("CARGO_FEATURE_EMBED_ICON").is_ok();

    if embed_icon && host.contains("linux") && target == "windows" {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let res_path = format!("{}/app.res", out_dir);
        // compile the .rc file
        let output = std::process::Command::new("x86_64-w64-mingw32-windres")
            .args(["scripts/app.rc", "-O", "coff", "-o", &res_path])
            .status()
            .expect("failed to run windres");

        assert!(output.success(), "windres failed");

        // Instruct rustc to link the resource
        println!("cargo:rustc-link-arg-bins={}", res_path);
    }
}
