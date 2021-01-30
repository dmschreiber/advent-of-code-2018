#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle20_test() {
    // super::print_things(&super::strip_outer(&"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".to_string()),"".to_string());
    println!();
    // super::print_things(&super::strip_outer(&"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".to_string()), "".to_string());
    // assert!(super::expand_regex(&"SSE(EE|N)".to_string()) == vec!["SSEEE", "SSEN"]);
    println!("{:?}",super::combos(&vec!["N".to_string(),"S".to_string()],&vec!["W".to_string(),"E".to_string()]));
    assert!(vec!["NW","NE","SW","SE"]==super::combos(&vec!["N".to_string(),"S".to_string()],&vec!["W".to_string(),"E".to_string()]));
    assert!(18==super::solve("./inputs/puzzle20-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle20_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle20.txt".to_string());
  }
}

use crate::common;
// use std::convert::TryInto;
// use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
  static ref INNER_REGEX: Regex = Regex::new(r"^(.*)\(([^\(\)]+)\)(.*)$").unwrap();
}


fn expand_regex(expression : &String) -> Vec<String> {
  let mut v = vec![];

  if let Some(inner) = INNER_REGEX.captures(expression) {
    for o in inner[2].split("|") {
      if o.len() == inner[2].split("|").map(|s| s.len()).max().unwrap() {
        v.push(format!("{}{}{}", &inner[1], o, &inner[3]));
      }
    }
  } else {
    v.push(expression.clone());
  }

  return v;
}

// fn print_vec(v : &Vec<String>) {
//   for i in v {
//     println!("{}", i);
//   }
// }

fn expansion_remains(v : &Vec<String>) -> bool {
  for item in v {
    if item.contains("|") {
      return true;
    }
  }
  return false;
}

pub fn max_expand(expression : &String) -> Vec<String> {
  let mut start = vec![expression.clone()];
  
  loop {
    let mut result = vec![];
    for item in &start {

      if item.contains("|") {
        let mut expanded = expand_regex(&item);
        result.append(&mut expanded);
      } else {
        result.push(item.to_string());
      }
      // println!("{:?}", timer.elapsed());
    }

    result.sort();
    result.dedup();
    // println!("remaining {}", result[0].as_bytes().iter().filter(|b| **b==b'|').count());
    // println!("--");
    // print_vec(&result);
    
    start = result;
    if !expansion_remains(&start) {
      break;
    }
    // println!("{}", start.len());
  }

  return start;
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

fn calculate_relative_destination(direction : &String, start : (isize,isize)) -> (isize,isize) {
  let mut p = start;

  for b in direction.as_bytes() {
    if *b == b'N' { p.1 = p.1 + 1; }
    if *b == b'S' { p.1 = p.1 - 1;  }
    if *b == b'E' { p.0 = p.0 + 1;  }
    if *b == b'W' { p.0 = p.0 - 1;  }
  }
  return p;
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
  expression(String),
  or(Vec<Thing>,Vec<Thing>),
}

pub fn print_things(things : &Vec<Thing>, depth : String) {
  if things.len() == 0 {
    println!("{}<nothing>", depth);
  }

  for t in things {
    match t {
      Thing::or(t1,t2) => { print_things(t1, format!(" OR {}", depth)); print_things(t2, format!(" OR {}", depth)); }
      Thing::expression(e) => { println!("{}{} - {}", depth, e, calculate_doors(e)); }
    }
  }
}

pub fn strip_outer(expression : &String) -> Vec<Thing> {
  let mut v = vec![];
  let mut buffer = vec![];
  let mut level = 0;
  let mut last_inner = "".to_string();

  for b in expression.as_bytes() {
    if *b == b'(' {
      level = level + 1;
      if level == 1 {
        v.push( Thing::expression( String::from_utf8(buffer.clone()).unwrap().to_string()));
        buffer.clear();
      } 
    } else if *b == b')' {
      level = level - 1;
      if level == 0 {
        let inner = String::from_utf8(buffer.clone()).unwrap().to_string();
        v.push(Thing::or(strip_outer(&last_inner), strip_outer(&inner) ));
        buffer.clear();
      } 
    } else if *b == b'|' && level == 1 {
      last_inner = String::from_utf8(buffer.clone()).unwrap().to_string();
      // v.push(Thing { expression : inner.clone(), children : strip_outer(&inner) });
      buffer.clear();
    }

    if *b == b'^' || *b == b'$' {

    } else if ( (*b == b'(' || *b == b'|' ) && level == 1) || (*b == b')' && level == 0) {

    } else {
      buffer.push(*b); 
    }
  }

  if buffer.len() > 0 {
    v.push(Thing::expression ( String::from_utf8(buffer).unwrap().to_string()));
  }
  return v;
}

fn find_furthest(things : &Vec<Thing>) -> String {
  let mut retval = "".to_string();

  for thing in things {
    match thing {
      Thing::or(thing1,thing2) => {
        let option1 = find_furthest(thing1);
        let option2 = find_furthest(thing2);
        // if option1.len() > option2.len() {
        if calculate_doors(&option1) > calculate_doors(&option2) {
          retval = retval + &option1;
        } else {
          retval = retval + &option2;
        }  
      }
      Thing::expression(e) => { retval = retval + &e; }
    }
  }
  return retval;
}

fn combos(l1 : &Vec<String>, l2 : &Vec<String>) -> Vec<String> {
  let mut v : Vec<String> = vec![];

  if l1.len() == 0 { return l2.clone(); }
  if l2.len() == 0 { return l1.clone(); }

  for i in l1 {
    for j in l2 {
      v.push(i.clone() + j);
    }
  }
  return v;
}

fn compare_directions(exp1 : &String, exp2 : &String) -> u32 {
  let length = std::cmp::min(exp1.len(),exp2.len());
  let mut count = 0;

  for i in 0..length {
    if exp1.as_bytes()[i] == exp2.as_bytes()[i] { count += 1; }
  }
  return count;
}

fn find_variations(things : &Vec<Thing>, starting_points : &Vec<(isize,isize)>, map : &mut HashMap<(isize,isize),u8>) -> Vec<String> {
  let mut retval : Vec<String> = vec![];

  for (_i,thing) in things.iter().enumerate() {
    let mut in_progress_starting_points = retval.iter().map(|d| calculate_destination(d)).collect::<Vec<(isize,isize)>>();
    in_progress_starting_points.dedup();
    if in_progress_starting_points.len() == 0 { 
      in_progress_starting_points = starting_points.clone(); 
    } else {
      for s in starting_points {
        for ipsp in in_progress_starting_points.iter_mut() {
          ipsp.0 = ipsp.0 + s.0;
          ipsp.1 = ipsp.1 + s.1;
        }
      }
    }
    println!("{} {:?}", _i+1, in_progress_starting_points);
    if in_progress_starting_points.len() > 1 { panic!("multiple in progress"); }
    match thing {
      Thing::or(thing1,thing2) => {
        let option1 = find_variations(&thing1,&in_progress_starting_points, map);
        let mut new_retval = combos(&retval, &option1);
  
        let option2 = find_variations(&thing2,&in_progress_starting_points, map);
        retval = combos(&retval, &option2);

        retval.append(&mut new_retval);  
      }
      Thing::expression(e) => {
        if retval.len() > 0 {
          retval = combos(&retval, &vec![e.clone()]);
        } else {
          retval.push(e.clone());
        }  
      }
    }

    for s in starting_points {
      // println!("starting at {:?}", s);
      for item in &retval {    
        for i in 0..=item.len() {
          let each_d = item[0..i].to_string();
          if i < item.len() {
            put_cango_spot(map, calculate_relative_destination(&each_d, *s), item.as_bytes()[i] as char);
          }
          if i > 0 {
            put_camefrom_spot(map, calculate_relative_destination(&each_d, *s), item.as_bytes()[i-1] as char);
          }
        }
      }
    }  
  }

  let mut reduce_map : HashMap<(isize,isize),(usize,String)> = HashMap::new();
  for s in starting_points {
    // println!("starting at {:?}", s);
    for item in &retval {
      // println!("starting at {:?} go {}", s, item);
      if let Some(existing) = reduce_map.get_mut(&calculate_relative_destination(&item,*s)) {
        let doors = item.len(); // calculate_doors(&d);
        if doors < existing.0 {
          existing.0 = doors;
          existing.1 = item.to_string();
        }
      } else {
        reduce_map.insert(calculate_relative_destination(&item,*s),(item.len(),item.clone()));
      }
  
      // for i in 0..=item.len() {
      //   let each_d = item[0..i].to_string();
      //   if i < item.len() {
      //     put_cango_spot(map, calculate_relative_destination(&each_d, *s), item.as_bytes()[i] as char);
      //   }
      //   if i > 0 {
      //     put_camefrom_spot(map, calculate_relative_destination(&each_d, *s), item.as_bytes()[i-1] as char);
      //   }
      // }
    }
  }

  // retval = reduce_map.values().map(|(_d,s)| s.to_string()).collect::<Vec<String>>();

  retval.sort_by_key(|k| calculate_doors(k) as isize * -1);
  if retval.len() > 50 {
    // draw_map(&map,(0,0));
  }
  println!("returning {}", retval.len());
  return retval;

}
// 1522 is too low
// 1586 not right (-23,26)
// 1553 not right (-23,26)
// Smallest to (17, -47) is 4142 (too high)

fn draw_map(map : &HashMap<(isize,isize),u8>, p : (isize,isize)) {
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
          line1 = line1 + "#-";
        } else {
          line1 = line1 + "##";
        }

        if *c & 0x08 == 0x08 {
          line2 = line2 + "|";
        } else {
          line2 = line2 + "#";
        }

        if x == 0 && y == 0 {
          line2 = line2 + "X";
        } else if x == p.0 && y == p.1 {
          line2 = line2 + "O";
        } else {
          line2 = line2 + ".";
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
  for l in lines {
    println!("Working {}", l[0..std::cmp::min(70,l.len())].to_string());
    let tree = strip_outer(&l);
    println!("{:?}",tree);
    // print_things(&tree,"".to_string());
    let d = find_furthest(&tree);

    println!("{:?} - {} to {:?}", d, calculate_doors(&d), calculate_destination(&d));
    let mut new_map = HashMap::new();
    let v = find_variations(&tree, &vec![(0,0)],&mut new_map);
    println!("{} variations", v.len());
    draw_map(&new_map, calculate_destination(&d));

    // let mut map = HashMap::new();
    // for item in &v {
    //   for i in 0..=item.len() {
    //     let each_d = item[0..i].to_string();
    //     if i < item.len() {
    //       put_cango_spot(&mut map, calculate_destination(&each_d), item.as_bytes()[i] as char);
    //     }
    //     if i > 0 {
    //       put_camefrom_spot(&mut map, calculate_destination(&each_d), item.as_bytes()[i-1] as char);
    //     }
    //   }
    // }
    // draw_map(&map, calculate_destination(&d));
    // println!("{:?}", map.keys().map(|which| (*which, other_path(&map, *which, (0,0)).unwrap().1 ) ).collect::<Vec<((isize,isize),isize)>>());
    // draw(&v[0]);
    println!("biggest astar {}", new_map.keys().map(|p| other_path(&new_map, *p, (0,0)).unwrap().1).max().unwrap());
    println!("astar path {:?}", other_path(&new_map,  calculate_destination(&v[0]), (0,0)).unwrap().1);
    println!("min to {:?} is {}", calculate_destination(&v[0]), calculate_doors(&v[0]));

    // let options = max_expand(&l);
    // let mut doors = 0;

    // // print_vec(&options.iter().map(|d| (calculate_doors(d),calculate_destination(d))).map(|doors| format!("{}->{:?}", doors.0,doors.1)).collect::<Vec<String>>());
    // let furthest = options.iter().map(|d| calculate_doors(d)).map(|doors| doors).max().unwrap();
    // // println!("furthest {}", furthest);
    // let which = options.iter().filter(|d| calculate_doors(d) == furthest).map(|s| calculate_destination(s)).collect::<Vec<(isize,isize)>>();
    // // println!("{:?}", which);

    // let mut smallest = 9999;
    // for dest in which {
    //   let candidate = options.iter().filter(|d| calculate_destination(d)==dest).map(|d| calculate_doors(d)).min().unwrap();
    //   if candidate < smallest {
    //     smallest = candidate;
    //     let local_direction = &options.iter().filter(|d| calculate_destination(d)==dest && calculate_doors(d) == smallest).map(|s| s.to_string()).collect::<Vec<String>>()[0];
    //     draw(&local_direction);
    //   }
    //   println!("Smallest to {:?} is {}", dest, smallest);
    // }
    
    // doors = smallest;
  }
    // return doors.try_into().unwrap();
  return 0;
}
