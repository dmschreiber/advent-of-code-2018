#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle19_test() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle19-test.txt".to_string());
  }

  #[test]
  pub fn puzzle19_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle19.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use std::time::Duration;
use std::thread;

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");
  thread::sleep(Duration::from_secs(10));

  let map = common::make_map(&lines);
  let spot = common::get_spot_on_map(&map, 0, 0, '.');
  assert!(spot == '.');
  
  return 0.try_into().unwrap();
}
