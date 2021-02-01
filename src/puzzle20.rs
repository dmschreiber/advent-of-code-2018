#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle20_test() {
    // super::print_things(&super::strip_outer(&"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".to_string()),"".to_string());
    println!();
    // super::print_things(&super::strip_outer(&"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".to_string()), "".to_string());
    // assert!(super::expand_regex(&"SSE(EE|N)".to_string()) == vec!["SSEEE", "SSEN"]);
    assert!(18==super::solve("./inputs/puzzle20-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle20_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle20.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
// use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;
use colored::*;

lazy_static! {
  static ref INNER_REGEX: Regex = Regex::new(r"^(.*)\(([^\(\)]+)\)(.*)$").unwrap();
}



#[allow(dead_code)]
fn draw(direction : &String) {
  let mut map = HashMap::new();

  let mut p = (0,0);

  for b in direction.as_bytes() {
    if *b == b'N' { p.1 = p.1 + 2; map.insert((p.0,p.1-1),'-'); }
    if *b == b'S' { p.1 = p.1 - 2;  map.insert((p.0,p.1+1),'-'); }
    if *b == b'E' { p.0 = p.0 + 2;  map.insert((p.0-1,p.1),'|'); }
    if *b == b'W' { p.0 = p.0 - 2;  map.insert((p.0+1,p.1),'|'); }
    map.insert((p.0,p.1), '.');
  }
  map.insert((p.0,p.1),'O');

  let min_x : isize = map.keys().map(|(x,_y)| *x).min().unwrap();
  let max_x : isize = map.keys().map(|(x,_y)| *x).max().unwrap();
  let min_y : isize = map.keys().map(|(_x,y)| *y).min().unwrap();
  let max_y : isize = map.keys().map(|(_x,y)| *y).max().unwrap();

  for y in (min_y..=max_y).rev() {
    for x in min_x..=max_x {
      if x==0 && y==0 {
        print!("X");
      } else if let Some(c) = map.get(&(x,y)) {
        print!("{}", c);
      } else {
        print!(" ");
      }
    }
    println!();
  }
}

fn calculate_destination(direction : &String) -> (isize,isize) {
  let mut p = (0,0);

  for b in direction.as_bytes() {
    if *b == b'N' { p.1 = p.1 + 1; }
    if *b == b'S' { p.1 = p.1 - 1;  }
    if *b == b'E' { p.0 = p.0 + 1;  }
    if *b == b'W' { p.0 = p.0 - 1;  }
  }
  return p;
}

fn calculate_doors(direction : &String) -> usize {
  let mut p = (0,0);
  let mut doors = vec![];

  for b in direction.as_bytes() {
    if *b == b'N' { p.1 = p.1 + 1; doors.push((p.0,p.1-1,0)); }
    if *b == b'S' { p.1 = p.1 - 1; doors.push((p.0,p.1,0)); }
    if *b == b'E' { p.0 = p.0 + 1; doors.push((p.0-1,p.1,1)); }
    if *b == b'W' { p.0 = p.0 - 1; doors.push((p.0,p.1,1)); }
  }
  doors.sort();
  doors.dedup();
  return doors.len();
}

#[derive(Debug,Clone)]
pub enum Thing {
  Expression(String),
  Or(Vec<Vec<Thing>>),
}

pub fn print_things(things : &Vec<Thing>, depth : String) {
  if things.len() == 0 {
    println!("{}<nothing>", depth);
  }

  for t in things {
    match t {
      Thing::Or(options) => { 
        for o in options {
          print_things(o, format!(" OR {}", depth));
        }
      }
      Thing::Expression(e) => { println!("{}{} - {}", depth, e, calculate_doors(e)); }
    }
  }
}

pub fn strip_outer(expression : &String) -> Vec<Thing> {
  let mut v = vec![];
  let mut buffer = vec![];
  let mut level = 0;
  let mut inner_or = vec![];

  for b in expression.as_bytes() {
    if *b == b'(' {
      if level == 0 {
        v.push( Thing::Expression( String::from_utf8(buffer.clone()).unwrap().to_string()));
        buffer.clear();
      } 
      level = level + 1;
    } else if *b == b')' {
      level = level - 1;
      if level == 0 {
        inner_or.push ( strip_outer(&String::from_utf8(buffer.clone()).unwrap().to_string()) );
        v.push(Thing::Or(inner_or));
        buffer.clear();
        inner_or = vec![];
      } 
    } else if *b == b'|' && level == 1 {

      inner_or.push( strip_outer(&String::from_utf8(buffer.clone()).unwrap().to_string() ) );
      // v.push(Thing { Expression : inner.clone(), children : strip_outer(&inner) });
      buffer.clear();
    }

    if *b == b'^' || *b == b'$' {

    } else if ( (*b == b'(' || *b == b'|' ) && level == 1) || (*b == b')' && level == 0) {

    } else {
      buffer.push(*b); 
    }
  }

  if buffer.len() > 0 {
    v.push(Thing::Expression ( String::from_utf8(buffer).unwrap().to_string()));
  }
  return v;
}

pub fn recreate(things : &Vec<Thing>) -> String {
  let mut retval = "".to_string();
  for thing in things {
    match thing {
      Thing::Or(options) => { retval = retval + &format!("({})", options.iter().map(|o| recreate(o)).map(|s| s.to_string()).collect::<Vec<String>>().join("|"));  }
      Thing::Expression(e) => { retval = retval + &e; }
    }
  }
  return retval;
}

fn find_furthest(things : &Vec<Thing>) -> String {
  let mut retval = "".to_string();

  for thing in things {
    match thing {
      Thing::Or(options) => {
        let mut result;
        let mut farthest_result = "".to_string();
        let mut farthest = 0;

        for o in options {
          result = find_furthest(o);
          if calculate_doors(&result) > farthest {
            farthest = calculate_doors(&result);
            farthest_result = result;
          }
        }
        retval = retval + &farthest_result;
      }
      Thing::Expression(e) => { retval = retval + &e; }
    }
  }
  return retval;
}


fn find_variations(things : &Vec<Thing>, starting_point : (isize,isize), map : &mut HashMap<(isize,isize),u8>) -> (isize,isize) {
  let mut current_pos = starting_point;

  for (_i,thing) in things.iter().enumerate() {
    match thing {
      Thing::Or(options) => {
        let mut resulting_pos = current_pos;
        for o in options {
          resulting_pos = find_variations(o, current_pos, map);
        }

        current_pos = resulting_pos;
      }
      Thing::Expression(e) => {
        for i in 0..e.len() {
          if i < e.len() {
            put_cango_spot(map, current_pos, e.as_bytes()[i] as char);
          }
          if i > 0 {
            put_camefrom_spot(map, current_pos, e.as_bytes()[i-1] as char);
          }
          match e.as_bytes()[i] {
            b'N' => { current_pos.1 = current_pos.1 + 1; }
            b'S' => { current_pos.1 = current_pos.1 - 1;  }
            b'E' => { current_pos.0 = current_pos.0 + 1;  }
            b'W' => { current_pos.0 = current_pos.0 - 1;  }
            _ => { panic!("misdirection"); }
          }
          if i < e.len() {
            put_camefrom_spot(map, current_pos, e.as_bytes()[i] as char);
          }
        }
      }
    }
  }

    // draw_map(&map,(0,0));
    return current_pos;

}
// 1522 is too low
// 1586 not right (-23,26)
// 1553 not right (-23,26)
// Smallest to (17, -47) is 4142 (too high)

#[allow(dead_code)]
fn draw_map(map : &HashMap<(isize,isize),u8>, p : Vec<(isize,isize)>) {
  let min_x : isize = map.keys().map(|(x,_y)| *x).min().unwrap();
  let max_x : isize = map.keys().map(|(x,_y)| *x).max().unwrap();
  let min_y : isize = map.keys().map(|(_x,y)| *y).min().unwrap();
  let max_y : isize = map.keys().map(|(_x,y)| *y).max().unwrap();
  let mut bottom = "".to_string();
  for y in (min_y..=max_y).rev() {
    let mut line1 = "".to_string();
    let mut line2 = "".to_string();
    bottom = "".to_string();
    for x in min_x..=max_x {
      bottom = bottom + "##";
      if let Some(c) = map.get(&(x,y)) {
        if *c & 0x01 == 0x01 {
          if p.contains(&(x,y)) {
            line1 = line1 + &format!("#{}", "-".to_string().red());
          } else {
            line1 = line1 + "#-";
          }
        } else {
          line1 = line1 + "##";
        }

        if *c & 0x08 == 0x08 {
          if p.contains(&(x,y)) {
            line2 = line2 + &format!("{}", "|".to_string().red());
          } else {
            line2 = line2 + "|";
          }
        } else {
          line2 = line2 + "#";
        }

        let spot;
        if (x,y) == (0,0) {
          spot = "X";
        } else if p.contains(&(x,y)){
          spot = "O";
        } else {
          spot = ".";
        }
        if p.contains(&(x,y)) {
          line2 = line2 + &format!("{}",spot.to_string().red());
        } else {
         line2 = line2 + spot;
        }

      } else {
        line1 = line1 + "  ";
        line2 = line2 + "  "
      }
    }
    println!("{}#", line1);
    println!("{}#", line2);
  }
  println!("{}#", bottom);  
}
fn put_camefrom_spot(map : &mut HashMap<(isize,isize),u8>, p : (isize,isize), direction : char) {
  let opposite = match direction {
    'N' => 'S',
    'S' => 'N',
    'E' => 'W',
    'W' => 'E',
    _ => { panic!("misdirection"); }
  };
  put_cango_spot(map, p, opposite);
}


fn put_cango_spot(map : &mut HashMap<(isize,isize),u8>, p : (isize,isize), direction : char) {
  let num =     match direction {
    'N' => { 0x01 }
    'E' => { 0x02 }
    'S' => { 0x04 }
    'W' => { 0x08 }
    _ => { panic!("misdirection");}
    };

  if let Some(existing) = map.get_mut(&p) {
    *existing = *existing | num;
  } else {
    map.insert(p,num);
  }
}

fn get_neighbors(map : &HashMap<(isize,isize),u8>, p : &(isize,isize)) -> Vec<(isize,isize)> {
  let mut v = vec![];

  if let Some(num) = map.get(p) {
    if num & 0x01 == 0x01 { v.push((p.0,p.1+1)); }
    if num & 0x02 == 0x02 { v.push((p.0+1,p.1)); }
    if num & 0x04 == 0x04 { v.push((p.0,p.1-1)); }
    if num & 0x08 == 0x08 { v.push((p.0-1,p.1)); }
  }
  return v;
}

pub extern crate pathfinding;

use pathfinding::prelude::astar;


fn manhattan_distance (p1 : (isize,isize), p2 : (isize,isize)) -> isize {
  return (p1.0-p2.0).abs()+(p1.1-p2.1).abs();
}
fn other_path(map : &HashMap<(isize,isize),u8>, 
          point_a : (isize,isize), 
          point_b : (isize,isize)) -> Option<(std::vec::Vec<(isize, isize)>, isize)> {

  let result : Option<(Vec<(isize,isize)>,isize)> = 
                    astar(&point_a, 
                    |p| get_neighbors(map, p).into_iter().map(|p| (p, 1)).collect::<Vec<((isize,isize),isize)>>(), 
                    |p| manhattan_distance(*p, point_b),
                    |p| *p == point_b);
  return result;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  let l = &lines[0];

  let tree = strip_outer(&l);
  assert!(l==&format!("^{}$",recreate(&tree)));

  let d = find_furthest(&tree);

  println!("{} to {:?}", calculate_doors(&d), calculate_destination(&d));
  let mut new_map = HashMap::new();
  find_variations(&tree, (0,0),&mut new_map);

  let farthest_room = other_path(&new_map,  calculate_destination(&d), (0,0)).unwrap();
  draw_map(&new_map, farthest_room.0);
  println!("longest shortest path {:?}", farthest_room.1);
  println!("{} rooms are >=1000 doors away", new_map.keys().map(|p| other_path(&new_map, *p, (0,0))).filter(|p| *p!=None ).map(|p| p.unwrap().1).filter(|p| *p >= 1000).count());

  return farthest_room.1.try_into().unwrap();
}
