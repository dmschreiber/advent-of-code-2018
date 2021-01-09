#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle3_test() {
    super::solve("./inputs/puzzle3-test.txt".to_string());
  }

  #[test]
  pub fn puzzle3_prod() {
    super::solve("./inputs/puzzle3.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");

  
  return 0.try_into().unwrap();
}
