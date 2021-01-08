#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle1() {
    assert!(common::format_binary(10)=="1010");
  }
}

use crate::common;
use std::convert::TryInto;

pub fn solve() -> i64 {
  let lines = common::read_input("./inputs/puzzle1.txt".to_string());

  let map = common::make_map(&lines);
  let spot = common::get_spot_on_map(&map, 0, 0, '.');
  assert!(spot == '.');
  return 0.try_into().unwrap();
}
