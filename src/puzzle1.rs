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

  return common::get_max_row(map).try_into().unwrap();
}
