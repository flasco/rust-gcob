use chrono::prelude::*;
use std::env;
use std::result::Result::{self, *};

static GIT_PREFIX: [&str; 3] = ["hotfix/", "feature/", "chore/"];

fn get_time_string() -> String {
    return Local::now().format("%Y%m%d").to_string();
}

fn branch_check(branch_name: &str) -> bool {
    for item in GIT_PREFIX {
        if branch_name.starts_with(item) {
            return true;
        }
    }
    return false;
}

fn get_args_input() -> Result<String, String> {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(name) => {
            if branch_check(&name) {
                Ok(name)
            } else {
                Err(format!(
                    "branch name not allowed, must start with {:?}",
                    GIT_PREFIX
                ))
            }
        }
        None => Err("Missing branch name".to_string()),
    }
}

pub fn generate_branch_name() -> Result<String, String> {
    return Ok(format!("{}_{}", get_args_input()?, get_time_string()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time_stamp_string() {
        assert_eq!(get_time_string().len() > 0, true);
    }

    #[test]
    fn test_arg_generate() {
      assert_eq!(branch_check("hotfix/asd"), true);
      assert_eq!(branch_check("hotfasdix/asd"), false);
      assert_eq!(branch_check("feature/asd"), true);
      assert_eq!(branch_check("featxure/asd"), false);
    }
}
