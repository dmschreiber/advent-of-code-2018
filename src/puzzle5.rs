#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle5_test() {
    assert!(super::calculate(&"aA".to_string())==0);
    assert!(super::calculate(&"dabAcCaCBAcCcaDA".to_string())==10);
    assert!(super::calculate(&super::remove(&"dabAcCaCBAcCcaDA".to_string(),'a'))==6);
    assert!(10==super::solve("./inputs/puzzle5-test.txt".to_string()));

  }

  #[test]
  pub fn puzzle5_prod() {
    assert!(11242==super::solve("./inputs/puzzle5.txt".to_string()));
    println!("part 1 {}", super::solve("./inputs/puzzle5.txt".to_string()));
  }
}

use crate::common;
use std::convert::TryInto;

// Problems in solving
// didn't clear my remaining_chars vector between iteration
// missed the last character when i was as at current_chars.len()-3

pub fn remove(s : &String, c : char) -> String {
  let mut ret_val = vec![];

  for current in s.as_bytes() {
    if *current as char != c && *current != (b'A' + (c as u8 - b'a')) {
      ret_val.push(current);
    }
  }
  // println!("{}", ret_val.iter().map(|b| **b as char).collect::<String>());
  return ret_val.iter().map(|b| **b as char).collect::<String>();
}

pub fn calculate(s : &String) -> u64 {

  let mut retval = 0;
  let mut skip_next = false;
  let mut current_chars = s.as_bytes().to_vec();
  let mut did_work = true;
  let mut iteration_count = 0;

  while did_work {
    iteration_count += 1;
    let mut remaining_chars = vec![];
    skip_next = false;
    if current_chars.len() == 0 { break; }
    for i in 0..current_chars.len()-1 {
      if !skip_next {
        let c = current_chars.get(i).unwrap();
        let c_next = current_chars.get(i+1).unwrap();
        // println!("{} {}", *c as char, *c_next as char);
        if ((*c >= b'a' && *c <= b'z') && (*c_next == (*c - b'a') + b'A')) || 
            ((*c >= b'A' && *c <= b'Z') && (*c_next == (c - b'A') + b'a')) {
          // println!("match {} {} {}", *c as char, *c_next as char, ((*c - b'a') + b'A') as char);
          skip_next = true;
          if current_chars.len() >= 3 && i == current_chars.len()-3 {
            remaining_chars.push(current_chars[current_chars.len()-1]);
          }
        } else {
          // println!("no match {} {} {}", *c as char, *c_next as char, ((*c - b'a') + b'A') as char);
          skip_next = false;
          remaining_chars.push(*c);
          if i == current_chars.len()-2 {
            remaining_chars.push(*c_next);

          }
        }
      } else {
        skip_next = false;
        // println!("Did work");
      }
    }
    // println!("Remaining chars {:?} - current chars {:?}", remaining_chars.iter().map(|c| *c as char).collect::<String>(), current_chars.iter().map(|c| *c as char).collect::<String>());
    // println!("Iteration count {} {} to  {}", iteration_count, current_chars.len(), remaining_chars.len());
    did_work = current_chars.len() != remaining_chars.len();
    current_chars = remaining_chars.clone();
  }

  return current_chars.len() as u64;

}
pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve {}", &lines[0][&lines[0].len()-5..]);

  let mut v = vec![];
  for c in 'a'..='z' {
    v.push((c,calculate(&remove(&lines[0],c))));
  }
  println!("minimum is {:?}", v.iter().min_by_key(|(a,b)| *b));
  return calculate(&lines[0]) as i64;  
}
