#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle19_test() {

  }

  #[test]
  pub fn puzzle19_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle19.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;
use crate::puzzle16;

lazy_static! {
  static ref IP_REGEX: Regex = Regex::new(r"^#ip ([0-9+]+)$").unwrap();
  static ref OPCODE_REGEX: Regex = Regex::new(r"^([a-z]+) ([0-9+]+) ([0-9+]+) ([0-9+]+)$").unwrap();
}

pub fn run (opcodes : &Vec<Vec<usize>>, initial_state : Vec<usize>, ip_register : usize) -> Vec<usize> {
  let mut ip = 0;
  let mut state = initial_state.clone();

  loop {
    let o = &opcodes[ip];
    //   // R1 = 1;
    //   // R4 = 1;
    //   // loop {
    //     // if R1*R4 == R5 {
    //     // R0 = R0 + R1;
    //     // } else {
    //     // R4 = R4 + 1; 
    //     // if R4 > R5 break;
    //     // }
    //   // }

    if ip == 2 && state[1] != 0 {
      for i in 1..=state[5] {
        state[1] = i;
        if state[5] % state[1] == 0 {
          state[0] = state[0] + state[1];
        }  
        state[4] = state[5];
        // state[1] = state[1] + 1;
        state[ip_register] = 15;
        ip = 15;

      }
    }

    state[ip_register] = ip;
    state = puzzle16::op_code(o[0], o[1..].to_vec(), &state);
    ip = state[ip_register];
    ip = ip + 1;
    state[ip_register] = ip;

    if ip >= opcodes.len() {break; }
  }
  return state;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  let ip_register;

  if let Some(parse_ip) = IP_REGEX.captures(&lines[0]) {
    ip_register = parse_ip[1].parse::<usize>().unwrap();
    println!("IP register {}", ip_register);
  } else {
    panic!("parse failure {}", lines[0]);
  }


  let mut opcodes = vec![];
  for l in lines[1..].iter() {
    if let Some(parse_opcode) = OPCODE_REGEX.captures(&l) {
      let opcode = puzzle16::OPCODES.iter().position(|k| **k==parse_opcode[1]).unwrap();
      let result = vec![opcode ,
                        parse_opcode[2].parse::<usize>().unwrap(),
                        parse_opcode[3].parse::<usize>().unwrap(),
                        parse_opcode[4].parse::<usize>().unwrap(),
                        ];
      opcodes.push(result); 
    } else {
      panic!("parse failure {}", l);
    }
  }

  let mut state = vec![0; 6];

  state = run(&opcodes,state,ip_register);

    println!("Day 19 part 1 {:?}", state[0]);

    let mut state = vec![0; 6];
    state[0] = 1;
    state = run(&opcodes, state, ip_register);

    //   // 10551367 + 1 + 2801 + 3767


    println!("Day 19 part 2 {:?}", state[0]);
    // 10551368 too low
    return state[0].try_into().unwrap();
}
