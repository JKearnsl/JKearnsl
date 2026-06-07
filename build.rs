use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");

    let hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string());

    let date = chrono::Utc::now().format("%Y.%m.%d %H:%M").to_string();

    println!("cargo:rustc-env=GIT_HASH={hash}");
    println!("cargo:rustc-env=BUILD_DATE={date}");
}
