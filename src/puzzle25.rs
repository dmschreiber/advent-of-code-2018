#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle25_test() {
    assert!(common::format_binary(10)=="1010");
    assert!((-3,-6,5,-2)==super::parse_point(&"-3,-6,5,-2".to_string()));
    assert!(4==super::solve("./inputs/puzzle25-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle25_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle25.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;

// pos=<41289914,12552653,-7638886>, r=70344373
lazy_static! {
  pub static ref POS_REGEX: Regex = Regex::new(r"^(-?[0-9+]+),(-?[0-9+]+),(-?[0-9+]+),(-?[0-9+]+)$").unwrap();
}

pub fn parse_point(expression : &String) -> (isize,isize,isize,isize) {
  if let Some(inner) = POS_REGEX.captures(expression) {
    let retval = (inner[1].parse::<isize>().unwrap(), inner[2].parse::<isize>().unwrap(), inner[3].parse::<isize>().unwrap(), inner[4].parse::<isize>().unwrap());
    return retval;
  }
  panic!("bad formatting {}", expression);
}

fn manhattan_distance(p1 : (isize,isize,isize,isize), p2 : (isize,isize,isize,isize)) -> isize {
  return (p1.0-p2.0).abs() + (p1.1-p2.1).abs() + (p1.2-p2.2).abs() + (p1.3-p2.3).abs();
}

fn does_fit(constellation : &Vec<(isize,isize,isize,isize)>, point : (isize,isize,isize,isize)) -> bool {
  for c in constellation {
    if manhattan_distance(*c, point) <= 3 {
      return true;
    }
  }
  return false;
}

fn already_joined(constellations : &Vec<Vec<(isize,isize,isize,isize)>>, point : (isize,isize,isize,isize)) -> bool {
  for c in constellations {
    for c_p in c {
      if point == *c_p {
        return true;
      }
    }
  }
  return false;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let mut constellations = vec![];
  let mut points = vec![];
  for l in lines {
    points.push(parse_point(&l));
  }

  loop {
    let mut found_fit = false;
    for p in &points {
      if already_joined(&constellations,*p) { continue; }
      for c in constellations.iter_mut() {
        if does_fit(c, *p) {
          c.push(*p);
          found_fit = true;
        }
      }
    }

    if found_fit == false { 
      for p in &points {
        if !already_joined(&constellations,*p) {
          constellations.push(vec![*p]);
          break;
        }
      }  
    }

    if points.iter().filter(|p| !already_joined(&constellations, **p)).count() == 0 {
      break;
    }
  }

  println!("constellations {}", constellations.len());
  return constellations.len().try_into().unwrap();
}
