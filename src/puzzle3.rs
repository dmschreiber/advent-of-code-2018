#[cfg(test)]
mod tests {
  #[test]
  pub fn puzzle3_test() {
    println!("{}", super::solve("./inputs/puzzle3-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle3_prod() {
    super::solve("./inputs/puzzle3.txt".to_string());
  }
}

use crate::common;
use std::collections::HashMap;
use regex::Regex;


lazy_static! {
  static ref CLAIM_REGEX: Regex = Regex::new(r"^#([0-9+]+) @ ([0-9+]+),([0-9+]+): ([0-9+]+)x([0-9+]+)$").unwrap();
}
pub struct Claim {
  id : u64,
  left : u64,
  top : u64,
  width : u64,
  height : u64,
}

pub fn solve(file_name : String) -> i64 {
  let overlapping_inches;

  let lines = common::read_input(file_name);
  println!("Start solve");
  let mut id;
  let mut left;
  let mut top;
  let mut width;
  let mut height;

  let mut claims = vec![];

  for l in lines {
    if let Some(args) = CLAIM_REGEX.captures(&l) {
      id = args[1].parse::<u64>().unwrap();
      left = args[2].parse::<u64>().unwrap();
      top = args[3].parse::<u64>().unwrap();
      width = args[4].parse::<u64>().unwrap();
      height = args[5].parse::<u64>().unwrap();
      let claim =  Claim { id : id, left : left, top : top, width: width, height : height };
      claims.push(claim);
    }
  }
    let mut map = HashMap::new();
    for c in &claims {
      for row in c.top+1..=c.top+c.height {
        for col in c.left+1..=c.left+c.width {
          if map.contains_key(&(row,col)) {
            // println!("Contains key");
            let count = map.get_mut(&(row,col)).unwrap();
            *count += 1;
          } else {
            map.insert((row,col),1);
          }
        }
      }
    }

    for c in &claims {
      let mut overlapping = false;
      for row in c.top+1..=c.top+c.height {
        for col in c.left+1..=c.left+c.width {
          if let Some(inch) = map.get(&(row,col)) {
              if *inch > 1 {
                overlapping = true;
              }
          }
        }
      }
      if !overlapping {
        println!("Claim id {} doeesn't overlap", c.id);
        break;
      }
    }

  overlapping_inches = map.values().filter(|c| **c > 1).count();
  println!("overlapping inches {}", overlapping_inches);
  return overlapping_inches as i64;
}
