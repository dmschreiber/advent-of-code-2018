#[cfg(test)]
mod tests {
  #[test]
  pub fn puzzle23_test() {
    assert!(((41289914,12552653,-7638886),70344373)==super::parse_pos(&"pos=<41289914,12552653,-7638886>, r=70344373".to_string()));
    assert!(7==super::solve("./inputs/puzzle23-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle23_prod() {
    super::solve("./inputs/puzzle23.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;

// pos=<41289914,12552653,-7638886>, r=70344373
lazy_static! {
  pub static ref POS_REGEX: Regex = Regex::new(r"^pos=<(-?[0-9+]+),(-?[0-9+]+),(-?[0-9+]+)>, r=([0-9+]+)$").unwrap();
}

fn parse_pos(expression : &String) -> ((isize,isize,isize),usize) {
  
  if let Some(args) = POS_REGEX.captures(expression) {
    let point = (args[1].parse::<isize>().unwrap(),args[2].parse::<isize>().unwrap(),args[3].parse::<isize>().unwrap());
    let radius = args[4].parse::<usize>().unwrap();
    return (point, radius);
  }

  panic!("bad format");
}

fn manhattan_distance(p1 : (isize,isize,isize), p2 : (isize,isize,isize)) -> usize {
  return ( (p1.0-p2.0).abs() + (p1.1-p2.1).abs() + (p1.2-p2.2).abs() ) as usize;
}
pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let nanobots = lines.iter().map(|l| parse_pos(l)).collect::<Vec<((isize,isize,isize),usize)>>();

  let biggest = nanobots.iter().map(|(_a,r)| *r).max().unwrap();
  let pos = nanobots.iter().filter(|(_a,r)| *r==biggest).map(|(a,_r)| *a).collect::<Vec<(isize,isize,isize)>>()[0];

  let in_range = nanobots.iter().filter(|(a,_r)| manhattan_distance(*a, pos) <= biggest).count();
  println!("{:?} - in range {}", pos, in_range);

  return in_range.try_into().unwrap();
}
