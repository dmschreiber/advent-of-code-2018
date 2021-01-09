#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle1_test() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle1-test.txt".to_string());
  }

  #[test]
  pub fn puzzle1_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle1.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");

  let mut history = vec![];
  let mut total = 0;
  for l in &lines {
    let num = l.parse::<i64>().unwrap();

    total += num;
    history.push(total);
  }
  println!("total = {}", total);
  let mut new_history = vec![];

  for n1 in &history {
    for n2 in &history {
      if n1 != n2 {
        if (n2-n1).abs() % total == 0 {
          if n1 > &total { new_history.push((n1,(n2-n1).abs()/total)); }
          if n2 > &total { new_history.push((n2, (n2-n1).abs()/total)); }
        }
      }
    }
  }
  let m = new_history.iter().map(|(_a,b)| b).min().unwrap();
  println!("try n={}, first repeated is {:?}", m, new_history.iter().filter(|(_a,b)| b==m).map(|(a,_b)| **a).collect::<Vec<i64>>()[0]);

  println!("{}", total);

  return total.try_into().unwrap();
}
