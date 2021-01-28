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
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;
use std::str;

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

fn print_vec(v : &Vec<String>) {
  for i in v {
    println!("{}", i);
  }
}

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
      let timer = Instant::now();
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

fn manhattan_distance (p1 : (isize,isize), p2 : (isize, isize)) -> isize {
  return (p2.1-p1.1).abs() + (p2.0-p1.0).abs();
}

#[derive(Debug,Clone)]
pub struct Thing {
  expression : String,
  children : Vec<Thing>,
  thing1 : Vec<Thing>,
  thing2 : Vec<Thing>,
}

pub fn print_things(things : &Vec<Thing>, depth : String) {
  for t in things {
    if t.children.len() != 0 {
      print_things(&t.children, format!("-{}", depth));
    } else if t.thing1.len() != 0 || t.thing2.len() != 0 {
      print_things(&t.thing1, format!(" OR {}", depth));
      print_things(&t.thing2, format!(" OR {}", depth));
    } else {
      println!("{}{} - {}", depth, t.expression, calculate_doors(&t.expression));
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
        v.push(Thing { expression : String::from_utf8(buffer.clone()).unwrap().to_string(), children : vec![], thing1 : vec![], thing2 : vec![] });
        buffer.clear();
      } 
    } else if *b == b')' {
      level = level - 1;
      if level == 0 {
        let inner = String::from_utf8(buffer.clone()).unwrap().to_string();
        v.push(Thing { expression : inner.clone(), children : vec![], thing1: strip_outer(&last_inner), thing2 : strip_outer(&inner) });
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
    v.push(Thing { expression : String::from_utf8(buffer).unwrap().to_string(), children : vec![] , thing1 : vec![], thing2 : vec![]});
  }
  return v;
}

fn find_furthest(things : &Vec<Thing>) -> String {
  let mut retval = "".to_string();

  for thing in things {
    if thing.children.len() != 0 {
      retval = retval + &find_furthest(&thing.children);
    } else if thing.thing1.len() > 0 || thing.thing2.len()> 0 {
      let option1 = find_furthest(&thing.thing1);
      let option2 = find_furthest(&thing.thing2);
      // if option1.len() > option2.len() {
      if calculate_doors(&option1) > calculate_doors(&option2) {
        retval = retval + &option1;
      } else {
        retval = retval + &option2;
      }
      
    } else {
      retval = retval + &thing.expression.clone();
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

fn find_variations(things : &Vec<Thing>) -> Vec<String> {
  let mut retval : Vec<String> = vec![];

  for thing in things {
    if thing.children.len() != 0 {
      let mut candidates = find_variations(&thing.children);
      if retval.len() > 0 {
        retval = combos(&retval, &candidates);
      } else {
        retval.append(&mut candidates);
      }

    } else if thing.thing1.len() > 0 || thing.thing2.len()> 0 {
      let option1 = find_variations(&thing.thing1);
      let mut new_retval = combos(&retval, &option1);

      let option2 = find_variations(&thing.thing2);
      retval = combos(&retval, &option2);
      retval.append(&mut new_retval);
    } else {
      if retval.len() > 0 {
        retval = combos(&retval, &vec![thing.expression.clone()]);
      } else {
        retval.push(thing.expression.clone());
      }
    }
  }

  let mut map : HashMap<(isize,isize),(usize,String)> = HashMap::new();
  for d in &retval {
    if let Some(existing) = map.get_mut(&calculate_destination(&d)) {
      let doors = calculate_doors(&d);
      if doors < existing.0 {
        existing.0 = doors;
        existing.1 = d.to_string();
      }
    } else {
      map.insert(calculate_destination(&d),(calculate_doors(&d),d.clone()));
    }
  }

  retval = map.values().map(|(d,s)| s.to_string()).collect::<Vec<String>>();
  retval.sort_by_key(|k| calculate_doors(k) as isize * -1);
  // println!("returning {}", retval.len());
  return retval;
  // if retval.len()> 0 {
    // for d in retval[0..std::cmp::min(retval.len(),2)].iter() {
    //   println!("farthest {:?}:{} (and {} other options)", 
    //   calculate_destination(d), 
    //   calculate_doors(d), retval.iter().filter(|which| calculate_destination(which)==calculate_destination(d)).count());
    // }
  // }

  // if retval.len() > cap {
  //   // println!("returning {:?}", retval[0]);
  //   return retval[0..cap].to_vec(); 
  // } else { 
  //   return retval;
  // }
}
// 1522 is too low
// 1586 not right (-23,26)
// 1553 not right (-23,26)
// Smallest to (17, -47) is 4142 (too high)

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  for l in lines {
    let tree = strip_outer(&l);
    print_things(&tree,"".to_string());
    let d = find_furthest(&tree);

    println!("{:?} - {} to {:?}", d, calculate_doors(&d), calculate_destination(&d));
    let v = find_variations(&tree);

    let mut map = HashMap::new();
    for each_d in &v {
      if let Some(existing) = map.get_mut(&calculate_destination(&each_d)) {
        let doors = calculate_doors(&each_d);
        if doors < *existing {
          *existing = doors;
        }
      } else {
        map.insert(calculate_destination(&d),calculate_doors(&each_d));
      }
    }
    println!("{} variations", map.len());
    println!("{:?}", map.keys()
                    .filter(|d| *map.get(d).unwrap() > 1500)
                    .map(|d| (*d,*map.get(d).unwrap())).collect::<Vec<((isize,isize),usize)>>());

    println!("{}", v[0]);
    let d1 = v.iter().filter(|which| calculate_destination(which)==calculate_destination(&d)).map(|which| calculate_doors(which)).min().unwrap();
    println!("min to {:?} is {}", calculate_destination(&d), d1);

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
