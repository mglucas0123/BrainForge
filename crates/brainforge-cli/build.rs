fn main() {
    let desc = std::process::Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "no-git".to_string());

    println!("cargo:rustc-env=BRAINFORGE_GIT_DESC={desc}");
    println!("cargo:rerun-if-changed=.git/HEAD");
}
