use std::process::Command;

pub fn checkout_branch(branch_name: &str) -> Result<String, String> {
  let res = Command::new("git")
    .arg("checkout")
    .arg("-b")
    .arg(branch_name)
    .output()
    .unwrap();

  println!("{:#?}", res);

  if res.stderr.len() > 0 {
    Err(String::from_utf8(res.stderr).unwrap())
  } else {
    Ok(String::from_utf8(res.stdout).unwrap())
  }
}
