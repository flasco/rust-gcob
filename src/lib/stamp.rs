use chrono::prelude::*;

pub fn get_time_string() -> String {
  return Local::now().format("%Y%m%d").to_string();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_string() {
    assert_eq!(get_time_string().len() > 0, true);
  }
}
