#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle2_test() {
    super::solve("./inputs/puzzle2-test.txt".to_string());
  }

  #[test]
  pub fn puzzle2_prod() {
    super::solve("./inputs/puzzle2.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use std::time::Duration;
use std::thread;

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");
  
  return 0.try_into().unwrap();
}
