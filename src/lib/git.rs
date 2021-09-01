use std::process::Command;

pub fn checkout_branch(branch_name: &str) -> Result<String, String> {
    let res = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output()
        .unwrap();

    if res.status.success() {
        Ok(format!("Switched to a new branch '{}'", branch_name))
    } else {
        Err(String::from_utf8(res.stderr).unwrap())
    }
}
