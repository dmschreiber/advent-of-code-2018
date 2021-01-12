#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle9_test() {
    assert!(common::format_binary(10)=="1010");
    assert!((10,1618)==super::get_number_between_text("10 players; last marble is worth 1618 points".to_string()));
    assert!(32==super::solve("9 players; last marble is worth 25 points".to_string()));
    assert!(8317==super::solve("10 players; last marble is worth 1618 points".to_string()));
    assert!(37305==super::solve("30 players; last marble is worth 5807 points".to_string()));
  }

  #[test]
  pub fn puzzle9_prod() {
    assert!(common::format_binary(10)=="1010");
    println!("Day 9 part 2 {}", super::solve("428 players; last marble is worth 70825 points".to_string()));
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
  static ref NUMBERS_REGEX: Regex = Regex::new(r"^([0-9+]+) players; last marble is worth ([0-9+]+) points$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn get_number_between_text(expression : String) -> (u32,u32) {

  if let Some(inner) = NUMBERS_REGEX.captures(&expression) {
    return (inner[1].parse::<u32>().unwrap(),inner[2].parse::<u32>().unwrap());
  }
  panic!("not a number");
}


pub fn solve(input : String) -> i64 {

  let (players,last_marble) = get_number_between_text(input);
  let high_score;
  
  let mut current_player = 1;
  let mut current_marble = 1;
  let mut scores = HashMap::new();

  let mut marbles = vec![0,1];
  for m in 2..=last_marble {
    if m % 23 == 0 {
      if let Some(score) = scores.get_mut(&current_player) {
        let additional_index = (current_marble - 7) % marbles.len();
        *score = *score + m;
        println!("Removing {} which is {}", additional_index, marbles.get(additional_index).unwrap());
        // println!("before {:?}", marbles);
        let additional_m = marbles.get(additional_index).unwrap().clone();
        *score = *score + additional_m;
        marbles.remove(additional_index);
        current_marble = additional_index;
        // println!("after {:?}", marbles);
      } else {
        let additional_index = (current_marble - 7) % marbles.len();
        println!("Removing {} which is {}", additional_index, marbles.get(additional_index).unwrap());
        // println!("before {:?}", marbles);
        let additional_m = marbles.get(additional_index).unwrap().clone();
        marbles.remove(additional_index);
        scores.insert(current_player,m+additional_m);
        current_marble = additional_index;
        // println!("after {:?}", marbles);
      }
    } else {
      let pre_size = marbles.len();
      let new_index = (current_marble+2) % pre_size;
      // println!("pre_size {}, new_index {}", pre_size, new_index);
      marbles.insert(new_index,m);
      current_marble = new_index;
  
    }
    // println!("[{}] ({}) {:?}", current_player+1, marbles[current_marble], marbles);
    current_player = (current_player + 1) % players;
  }

  println!("{:?}", scores);
  high_score = *scores.values().max().unwrap();
  return high_score.try_into().unwrap();
}
