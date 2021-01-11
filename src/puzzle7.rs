#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle7_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(super::get_steps("Step C must be finished before step A can begin.".to_string())==('C','A'));
    assert!("CABDFE".to_string()==super::find_order("./inputs/puzzle7-test.txt".to_string()));
    println!("part 2 result {}", super::solve("./inputs/puzzle7-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle7_prod() {
    assert!(common::format_binary(10)=="1010");
    println!("part 1 result {}", super::find_order("./inputs/puzzle7.txt".to_string()));
    println!("part 2 result {}", super::solve("./inputs/puzzle7.txt".to_string()));
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

fn load_all_steps(lines : Vec<String>) -> HashMap<char,Vec<char>> {
  let mut list : HashMap<char,Vec<char>> = HashMap::new();
  for l in &lines {
    
    let step = get_steps(l.to_string());
    if let Some(v) = list.get_mut(&step.0) {
      v.push(step.1);
    } else {
      list.insert(step.0, vec![step.1]);
    }
  }

  return list;
}
pub fn find_order(file_name : String) -> String {
  let mut ret_val = "".to_string();
  let lines = common::read_input(file_name);

  let mut list = load_all_steps(lines);

  let mut available_chars = list.keys().filter(|c| get_prerequ(**c, &list).len() == 0).map(|c| *c).collect::<Vec<char>>();

  let mut resulting_order : Vec<char>= vec![];

  while available_chars.len() > 0 {
    available_chars.sort();

    let mut char_index = 0;
    let mut next_char = available_chars[char_index];
    while get_prerequ(next_char, &list).iter().filter(|target_char| !resulting_order.contains(target_char)).count() > 0 {
      // println!("Prerequ of {} are {:?}", next_char, get_prerequ(next_char, &list));
      char_index += 1;
      next_char = available_chars[char_index].clone();
    }
    available_chars.remove(char_index);

    resulting_order.push(next_char);
    let subsequent;
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
  let mut current_minute = 0;

  let lines = common::read_input(file_name.to_string());
  let order = find_order(file_name.to_string());
  let mut steps_remain : Vec<char> = order.as_bytes().iter().map(|c| *c as char).collect();
  let steps_map = load_all_steps(lines);


  let num_workers = 5;
  let initial_time = 60;

  let mut workers : Vec<Option<u32>> = vec![None; num_workers];
  let mut work : Vec<char> = vec!['0'; num_workers];

  while steps_remain.len() > 0 || work.iter().filter(|c| **c != '0').count() > 0 {
    println!("second {}", current_minute);

    for position in 0..workers.len() {
      if let Some(w) = workers.get_mut(position).unwrap() {
        // decrement
        *w = *w - 1;
      } else if steps_remain.len() > 0 {
        let mut char_index = 0;
        while get_prerequ(steps_remain[char_index] as char, &steps_map).iter().filter(|pre_char| steps_remain.contains(&(**pre_char)) || work.contains(pre_char)).count() > 0 {
          char_index = char_index + 1;
          if char_index == steps_remain.len() { break; }
        }
        if char_index < steps_remain.len() {
          println!("assign work {}", steps_remain[char_index]);
          *workers.get_mut(position).unwrap() = Some(( initial_time + steps_remain[char_index] as u8 - b'A' ) as u32);
          work[position] = steps_remain[char_index];
          steps_remain.remove(char_index);
          // assign work
        }
      }
    }

    for position in 0..workers.len() {
       let p = workers.get_mut(position).unwrap();
       if Some(0) == *p {
         *p = None;
         work[position] = '0';
       }
    }
    current_minute = current_minute + 1;
  }
 
  // 1007 too low
  // 1070 too high

  return current_minute.try_into().unwrap();
}
