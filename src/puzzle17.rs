#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle17_test() {
    assert!(57==super::solve("./inputs/puzzle17-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle17_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle17.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;
use std::collections::HashMap;

// RegEx
lazy_static! {
  static ref XY_REGEX: Regex = Regex::new(r"^x=([0-9+]+), y=([0-9+]+)\.\.([0-9+]+)$").unwrap();
  static ref YX_REGEX: Regex = Regex::new(r"^y=([0-9+]+), x=([0-9+]+)\.\.([0-9+]+)$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn get_number_range(expression : String) -> Vec<(usize,usize)> {
  let mut v = vec![];

  if let Some(inner) = XY_REGEX.captures(&expression) {
    let x = inner[1].parse::<usize>().unwrap();
    let y1 = inner[2].parse::<usize>().unwrap();
    let y2 = inner[3].parse::<usize>().unwrap();

    for y in y1..=y2 {
      v.push((x,y));
    }
    return v;
  } else if let Some(inner) = YX_REGEX.captures(&expression) {
    let y = inner[1].parse::<usize>().unwrap();
    let x1 = inner[2].parse::<usize>().unwrap();
    let x2 = inner[3].parse::<usize>().unwrap();

    for x in x1..=x2 {
      v.push((x,y));
    }
    return v;
  } else {
    panic!("not a number [{}]", expression);
  }
}

fn print_map(map : &HashMap<(usize,usize),char>) {
  let max_y = map.keys().map(|(_x,y)| *y).max().unwrap();
  let min_y = map.keys().map(|(_x,y)| *y).min().unwrap();

  let max_x = map.keys().map(|(x,_y)| *x).max().unwrap();
  let min_x = map.keys().map(|(x,_y)| *x).min().unwrap();

  for y in min_y..=max_y {
    for x in min_x..=max_x {
      if let Some(c) = map.get(&(x,y)) {
        print!("{}", c);
      } else {
        print!(".");
      }
    }
    println!();
  }
}

fn get_spot(map : &HashMap<(usize,usize),char>, spot : (usize,usize)) -> char {
  if let Some(result) = map.get(&spot) {
    return *result;
  } else {
    return '.';
  }
}

fn put_spot(map : &mut HashMap<(usize,usize),char>, spot : (usize,usize), val : char) {
  if let Some(p) = map.get_mut(&spot) {
    *p = val;
  } else {
    map.insert(spot, val);
  }
}

fn flow_down(map : &mut HashMap<(usize,usize),char>, point : (usize,usize)) -> (usize,usize) {
  let mut next_spot = (point.0,point.1+1);
  let max_y = map.keys().map(|(_x,y)| *y).max().unwrap();
  if next_spot.1 > max_y { return (next_spot.0,next_spot.1-1); }

  loop {
    if  let Some(p) = map.get_mut(&next_spot) {
      if *p == '#' { return (next_spot.0,next_spot.1-1); }
      else if *p == '.' {
        *p = '|';
      }
    } else {
      map.insert(next_spot, '|');
    }

    next_spot = (next_spot.0, next_spot.1+1);
    if next_spot.1+1 > max_y { return (next_spot.0,next_spot.1-1);  }
  }
}

fn can_fall(map : &HashMap<(usize,usize),char>, point : (usize,usize)) -> bool {
  if get_spot(map,point) != '|' {
    return false;
  }
  if get_spot(map,(point.0,point.1+1)) == '.' {
    return true;
  }
  return false;
}

fn contained(map : &HashMap<(usize,usize),char>, point : (usize,usize)) -> bool {
  if get_spot(map, point) != '|' {
    return false;
  }

  if can_fall(map, point) {
    return false;
  } 


  let mut target_spot = (point.0+1,point.1);
  loop {

    let spot = get_spot(map, target_spot);
    if spot == '|' || spot == '~' {
      target_spot = (target_spot.0+1,target_spot.1);
    } else if spot == '#' {
      break;
    } else {
      return false;
    }
  }
  let mut target_spot = (point.0-1,point.1);
  loop {

    let spot = get_spot(map, target_spot);
    if spot == '|' || spot == '~' {
      target_spot = (target_spot.0-1,target_spot.1);
    } else if spot == '#' {
      break;
    } else {
      return false;
    }
  }
  return true;
}

fn can_spread(map : &HashMap<(usize,usize),char>, point : (usize,usize)) -> bool {
  if get_spot(map,point) != '|' {
    return false;
  }

  if get_spot(map,(point.0,point.1+1)) != '#' && get_spot(map,(point.0,point.1+1)) != '~' {
    return false;
  }

  if get_spot(map,(point.0-1,point.1)) == '.' {
    return true;
  }
  if get_spot(map,(point.0+1,point.1)) == '.' {
    return true;
  }
  return false;

}
fn spread_out_point(mut map : &mut HashMap<(usize,usize),char>, p : (usize,usize) ) {
  if get_spot(&map, (p.0+1,p.1)) =='.' {
    put_spot(&mut map, (p.0+1,p.1), '|');
    if can_spread(map,(p.0+1,p.1)) { spread_out_point(&mut map, (p.0+1,p.1)); }
  }
  if get_spot(&map, (p.0-1,p.1)) =='.' {
    put_spot(&mut map, (p.0-1,p.1), '|');
    if can_spread(map,(p.0-1,p.1)) { spread_out_point(&mut map, (p.0-1,p.1)); } 
  }
}
fn spread_out(mut map : &mut HashMap<(usize,usize),char>) {

  loop {
    let points = map.keys().filter(|k| can_spread(map,**k)).map(|k| *k).collect::<Vec<(usize,usize)>>();
    if points.len() == 0 { break; }

    for p in points {
      spread_out_point(&mut map, p)
    }
  }
}

fn settle_out(mut map : &mut HashMap<(usize,usize),char>) {

  let points = map.keys().filter(|k| contained(&map, **k)).map(|k| *k).collect::<Vec<(usize,usize)>>();

  for p in points {
      put_spot(&mut map, p, '~');
  }

}

fn fall(mut map : &mut HashMap<(usize,usize),char>) {
  let points = map.keys().filter(|k| can_fall(&map, **k)).map(|k| *k).collect::<Vec<(usize,usize)>>();

  for p in points {
      flow_down(&mut map, p);
  }

}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let mut map = HashMap::new();
  for l in lines {
    for p in get_number_range(l) {
      map.insert(p,'#');
    }
  }

  let max_y = map.keys().map(|(_x,y)| *y).max().unwrap();
  let min_y = map.keys().map(|(_x,y)| *y).min().unwrap();

  let max_x = map.keys().map(|(x,_y)| *x).max().unwrap();
  let min_x = map.keys().map(|(x,_y)| *x).min().unwrap();
  let spring = (500,min_y-1);

  println!("x min/max {}/{} - y min/max {}/{}", min_x,max_x,min_y,max_y);
  // print_map(&map);
  flow_down(&mut map, spring);

  let mut water_count = (map.values().filter(|v| **v=='|').count(), map.values().filter(|v| **v=='~').count());
  loop {
    spread_out(&mut map);
    settle_out(&mut map);
    fall(&mut map);
    // print_map(&map);

    if water_count != (map.values().filter(|v| **v=='|').count(), map.values().filter(|v| **v=='~').count()) {
      water_count = (map.values().filter(|v| **v=='|').count(), map.values().filter(|v| **v=='~').count());
    } else {
      break;
    }
  }

  print_map(&map);
  println!("water count {:?} (at rest water {})", water_count.0+water_count.1, water_count.1);
  // water count 31162 too high
  return (water_count.0+water_count.1).try_into().unwrap();
}
