#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle6_test() {

    assert!(super::get_coordinates("1, 1".to_string())==(1,1));
    assert!(super::solve("./inputs/puzzle6-test.txt".to_string())==17);
  }

  #[test]
  pub fn puzzle6_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle6.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;
use std::collections::HashMap;


// Problems
// - removing the spots that were equistant
// - detecting the *minimum* equidistant not just equidisstant

lazy_static! {
  static ref COORDINATES_REGEX: Regex = Regex::new(r"^([0-9+]+), ([0-9+]+)$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn get_coordinates(expression : String) -> (isize,isize) {

  if let Some(inner) = COORDINATES_REGEX.captures(&expression) {
    let x = inner[1].parse::<isize>().unwrap();
    let y = inner[2].parse::<isize>().unwrap();
    return (x,y);
  }
  panic!("not a number");
}

pub fn manhattan_distance(p1 : (isize,isize), p2 : (isize,isize)) -> isize {
  return (p1.0-p2.0).abs() + (p1.1-p2.1).abs();
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");

  let mut coordinates = vec![];
  for l in lines {
    coordinates.push(get_coordinates(l));
  }

  let min_x = coordinates.iter().map(|(x,_y)| *x).min().unwrap();
  let max_x = coordinates.iter().map(|(x,_y)| *x).max().unwrap();
  let min_y = coordinates.iter().map(|(_x,y)| *y).min().unwrap();
  let max_y = coordinates.iter().map(|(_x,y)| *y).max().unwrap();

  let mut map : HashMap<(isize,isize),((isize,isize),isize)> = HashMap::new();

  let threshold_size = 10000;
  let mut region_size = 0;

  for x in min_x..=max_x {
    for y in min_y..=max_y {
      let mut total_distance = 0;
      for p in &coordinates {
        let d = manhattan_distance(*p, (x,y));
        total_distance = total_distance + d;

        if let Some(place) = map.get_mut(&(x,y)) {
          if d < place.1 {
            place.0 = *p;
            place.1 = d;
          }
        } else {
          map.insert((x,y),(*p,d));
        }
      }

      if total_distance < threshold_size {
        region_size = region_size + 1;
      }

      if coordinates.iter().filter(|new_p| **new_p != map.get(&(x,y)).unwrap().0 && manhattan_distance(**new_p, (x,y)) == map.get(&(x,y)).unwrap().1).count() > 0 {
        map.remove(&(x,y));
      }  
      
    }
  }
  

  let mut biggest = 0;
  let mut which = (0,0);
  for c in &coordinates {
    let area = map.values().filter(|(p,_d)| *p == *c).count();
    if area > biggest {
      biggest = area;
      which = *c;
    }
  }

  println!("biggest is {:?} size {}", which, biggest);
  println!("Region size with threshold {} is {}", threshold_size, region_size);
  return biggest.try_into().unwrap();

  // part 2 - 36238

}
