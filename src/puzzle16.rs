#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle16_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(super::op_code(2, vec![2,1,2],&vec![3,2,1,1]) == vec![3,2,2,1]);
    assert!(super::op_code(1, vec![2,1,2],&vec![3,2,1,1]) == vec![3,2,2,1]);
    assert!(super::op_code(9, vec![2,1,2],&vec![3,2,1,1]) == vec![3,2,2,1]);
    // assert!(1==super::solve("./inputs/puzzle16-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle16_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle16.txt".to_string());
  }
}
use std::fs;
use std::collections::HashMap;
use std::convert::TryInto;
use regex::Regex;

pub const OPCODES: [&'static str; 16] = ["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"];

pub fn op_code ( op_code : usize, args : Vec<usize>, state : &Vec<usize>) -> Vec<usize> {
  let mut retval = state.clone();
  let (reg_a, reg_b, reg_c) = (args[0], args[1], args[2]);

  match op_code {
    0 => { retval[reg_c] = state[reg_a]+state[reg_b]; }
    1 => { retval[reg_c] = state[reg_a] + reg_b; }
    2 => { retval[reg_c] = state[reg_a] * state[reg_b]; }
    3 => { retval[reg_c] = state[reg_a] * reg_b; }
    4 => { retval[reg_c] = state[reg_a] & state[reg_b]; }
    5 => { retval[reg_c] = state[reg_a] & reg_b; }
    6 => { retval[reg_c] = state[reg_a] | state[reg_b]; }
    7 => { retval[reg_c] = state[reg_a] | reg_b; }
    8 => { retval[reg_c] = state[reg_a]; }
    9 => { retval[reg_c] = reg_a; }
    10 => { if reg_a > state[reg_b] { retval[reg_c] = 1; } else { retval[reg_c] = 0; } }
    11 => { if state[reg_a] > reg_b { retval[reg_c] = 1; } else { retval[reg_c] = 0; } }
    12 => { if state[reg_a] > state[reg_b] { retval[reg_c] = 1; } else { retval[reg_c] = 0; } }
    13 => { if reg_a == state[reg_b] { retval[reg_c] = 1; } else { retval[reg_c] = 0; } }
    14 => { if state[reg_a] == reg_b { retval[reg_c] = 1; } else { retval[reg_c] = 0; } }
    15 => { if state[reg_a] == state[reg_b] { retval[reg_c] = 1; } else { retval[reg_c] = 0; } }

    _ => { panic!("invalid opcode"); }
  }

  return retval;
}

lazy_static! {
  static ref STATE_REGEX: Regex = Regex::new(r"^.+ \[([0-9+]+), ([0-9+]+), ([0-9+]+), ([0-9+]+)\]$").unwrap();
  static ref OPCODE_REGEX: Regex = Regex::new(r"^([0-9+]+) ([0-9+]+) ([0-9+]+) ([0-9+]+)$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn get_vec(expression : &String) -> Vec<usize> {

  if let Some(inner) = STATE_REGEX.captures(&expression) {
    return vec![inner[1].parse::<usize>().unwrap(), inner[2].parse::<usize>().unwrap(), inner[3].parse::<usize>().unwrap(), inner[4].parse::<usize>().unwrap()];
  } else if let Some(inner) = OPCODE_REGEX.captures(&expression) {
    return vec![inner[1].parse::<usize>().unwrap(), inner[2].parse::<usize>().unwrap(), inner[3].parse::<usize>().unwrap(), inner[4].parse::<usize>().unwrap()];
  }
  panic!("not a number [{}]", expression);
}

pub fn read_input(filename: String) -> (Vec<String>,Vec<String>) {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let sections: Vec<String> = contents.split("\n\n\n\n").map(|s| (&*s).to_string() ).collect();

  let samples: Vec<String> = sections[0].split("\n\n").map(|s| (&*s).to_string() ).collect();
  let instructions: Vec<String> = sections[1].split("\n").map(|s| (&*s).to_string() ).collect();

  (samples,instructions)
}

pub fn solve(file_name : String) -> i64 {
  let (samples,instructions) = read_input(file_name);

  let result;
  let mut which_work = vec![];

  for s in &samples {
    let lines : Vec<String> = s.split("\n").map(|s| (&*s).to_string() ).collect();
    let before = get_vec(&lines[0]);
    let opcodes = get_vec(&lines[1]);
    let after = get_vec(&lines[2]);
    let mut works_count = 0;

    for i in 0..OPCODES.len() {
      if op_code(i,opcodes[1..].to_vec(),&before) == after {
        works_count = works_count + 1;
      }
    }
    if works_count >= 3 {
      which_work.push(opcodes);
    }
  }
  result = which_work.len();

  let mut op_code_map = HashMap::new();

  while op_code_map.len() < OPCODES.len() {
    for s in &samples {
      let lines : Vec<String> = s.split("\n").map(|s| (&*s).to_string() ).collect();
      let before = get_vec(&lines[0]);
      let opcodes = get_vec(&lines[1]);
      let after = get_vec(&lines[2]);
      let mut works_count = 0;
      let mut which_worked = 99;

      for i in 0..OPCODES.len() {
        if op_code_map.values().filter(|opcode| **opcode==i).count() == 0 {
          if op_code(i,opcodes[1..].to_vec(),&before) == after {
            works_count = works_count + 1;
            which_worked = i;
          }
        }
      }
      if works_count == 1 {
        // println!("Found {}", OPCODES[which_worked]);
        op_code_map.insert(opcodes[0],which_worked);
      }
    }
  }

  let mut state = vec![0,0,0,0];

  for i in instructions {
    let opcodes = get_vec(&i);
    state = op_code(*op_code_map.get(&opcodes[0]).unwrap(), opcodes[1..].to_vec(), &state);
  }

  println!("Day 16 part 1 {}", result);
  println!("Day 16 part 2 {:?}", state); 
  return result.try_into().unwrap();
}
