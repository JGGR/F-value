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

    assert!(output.status.success());

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_branch_name() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();

    assert!(output.status.success());

    String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string()
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
}
