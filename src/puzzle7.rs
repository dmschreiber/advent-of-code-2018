#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle7_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(super::get_steps("Step C must be finished before step A can begin.".to_string())==('C','A'));
    assert!("CABDFE".to_string()==super::find_order("./inputs/puzzle7-test.txt".to_string()));
    // super::solve("./inputs/puzzle7-test.txt".to_string());
  }

  #[test]
  pub fn puzzle7_prod() {
    assert!(common::format_binary(10)=="1010");
    println!("part 1 result {}", super::find_order("./inputs/puzzle7.txt".to_string()));
    // super::solve("./inputs/puzzle7.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
  static ref STEPS_REGEX: Regex = Regex::new(r"^Step (.*) must be finished before step (.*) can begin.$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn get_steps(expression : String) -> (char,char) {

  if let Some(inner) = STEPS_REGEX.captures(&expression) {
    return (inner[1].to_string().as_bytes()[0] as char, inner[2].to_string().as_bytes()[0] as char);

  }
  panic!("no match");
}

pub fn get_prerequ(c : char, steps : &HashMap<char,Vec<char>>) -> Vec<char> {
  let mut v = vec![];

  for s in steps.keys() {
    if steps.get(s).unwrap().contains(&c) {
      v.push(*s);
    }
  }
  return v;
}

pub fn find_order(file_name : String) -> String {
  let mut ret_val = "".to_string();
  let lines = common::read_input(file_name);

  let mut list : HashMap<char,Vec<char>> = HashMap::new();
  for l in &lines {
    
    let step = get_steps(l.to_string());
    if let Some(v) = list.get_mut(&step.0) {
      v.push(step.1);
    } else {
      list.insert(step.0, vec![step.1]);
    }
  }

  let mut available_chars = vec![];

  let mut start_char = '0';
  for (i,c) in list.keys().enumerate() {
    let mut found = false;
    for (j,other) in list.values().enumerate() {
      if i != j {
        if other.contains(c) {
          found = true;
          break;
        }
      }
    }
    if !found {
      println!("no prerequs {}", *c);
      available_chars.push(*c);
      start_char = *c;
    }
  }

  println!("start char {} - {:?}", start_char, available_chars);

  let mut resulting_order : Vec<char>= vec![];

  while available_chars.len() > 0 {
    available_chars.sort();

    let mut char_index = 0;
    let mut next_char = available_chars[char_index];
    while get_prerequ(next_char, &list).iter().filter(|target_char| !resulting_order.contains(target_char)).count() > 0 {
      println!("Prerequ of {} are {:?}", next_char, get_prerequ(next_char, &list));
      char_index += 1;
      next_char = available_chars[char_index].clone();
    }
    available_chars.remove(char_index);

    resulting_order.push(next_char);
    let mut subsequent;
    if let Some(thing) = list.get_mut(&next_char) {
      subsequent = thing.clone();
    } else {
      subsequent = vec![];
    }
    for subsequent_char in subsequent {
      if !resulting_order.contains(&&subsequent_char) {
        available_chars.push(subsequent_char);
      }
    }

    available_chars.sort();
    available_chars.dedup();

    println!("{:?} -> {:?}", available_chars, resulting_order);
  }

  ret_val = resulting_order.iter().map(|my_char| *my_char).collect::<String>();
  return ret_val;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");

  let map = common::make_map(&lines);
  let spot = common::get_spot_on_map(&map, 0, 0, '.');
  assert!(spot == '.');
  
  return 0.try_into().unwrap();
}
