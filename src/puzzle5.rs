#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle5_test() {
    assert!(super::calculate(&"aA".to_string())==0);
    assert!(super::calculate(&"dabAcCaCBAcCcaDA".to_string())==10);
  }

  #[test]
  pub fn puzzle5_prod() {
    assert!(common::format_binary(10)=="1010");
    println!("part 1 {}", super::solve("./inputs/puzzle5.txt".to_string()));
  }
}

use crate::common;
use std::convert::TryInto;

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
        if *c >= b'a' && *c <= b'z'  {
          if *c_next == (*c - b'a') + b'A' {
            // println!("match {} {} {}", *c as char, *c_next as char, ((*c - b'a') + b'A') as char);
            skip_next = true;
            if i == current_chars.len()-3 {
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
        } else if *c >= b'A' && *c <= b'Z' {
          if *c_next == (c - b'A') + b'a' {
            // println!("match {} {}", *c as char, *c_next as char);
            skip_next = true;
            if i == current_chars.len()-3 {
              remaining_chars.push(current_chars[current_chars.len()-1]);
            }

          } else {
            // println!("no match {} {}", *c as char, *c_next as char);
            skip_next = false;
            remaining_chars.push(*c);
            if i == current_chars.len()-2 {
              remaining_chars.push(*c_next);

            }
          }
        } else {
          panic!("Should get here");
        }
      } else {
        skip_next = false;
        // println!("Did work");
      }
    }
    // println!("Remaining chars {:?} - current chars {:?}", remaining_chars.iter().map(|c| *c as char).collect::<String>(), current_chars.iter().map(|c| *c as char).collect::<String>());
    println!("Iteration count {} {} to  {}", iteration_count, current_chars.len(), remaining_chars.len());
    did_work = current_chars.len() != remaining_chars.len();
    current_chars = remaining_chars.clone();
  }

  return current_chars.len() as u64;

}
pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve {}", &lines[0][&lines[0].len()-5..]);

  return calculate(&lines[0]) as i64;  
  // 11240 too low
}
