#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle19_test() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle19-test.txt".to_string());
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
  let mut ip = 0;
  let mut counter = 0;

  loop {
    let o = &opcodes[ip];
    state[ip_register] = ip;
    state = puzzle16::op_code(o[0], o[1..].to_vec(), &state);
    ip = state[ip_register];
    ip = ip + 1;
    state[ip_register] = ip;
    counter = counter + 1;
    if ip >= opcodes.len() {break; }
  }
    println!("Day 19 part 1 {:?} ({})", state[0], counter);

    let mut state = vec![0; 6];
    state[0] = 1;
    let mut ip = 0;
    let mut counter = 0;

    loop {
      // print!("ip={}, {:?} ", ip, state);
      let o = &opcodes[ip];

      // R1 = 1;
      // R4 = 1;
      // loop {
        // if R1*R4 == R5 {
        // R0 = R0 + R1;
        // } else {
        // R4 = R4 + 1; 
        // if R4 > R5 break;
        // }
      // }
      // 10551367 + 1 + 2801 + 3767


      if ip == 2 && state[1] == 1 {
        state[0] = state[0] + 1;
        state[1] = 2;
        state[4] = state[5];
        state[ip_register] = 12;
        ip = 12;

      }
      else if ip == 2 && state[1] != 0 {
        // println!("{:?}", state);
        if state[5] % state[1] == 0 {
          state[0] = state[0] + state[1];
        }  
        state[4] = state[5];
        state[1] = state[1] + 1;
        state[ip_register] = 12;
        ip = 12;
      }

      state[ip_register] = ip;
      state = puzzle16::op_code(o[0], o[1..].to_vec(), &state);
      ip = state[ip_register];
      ip = ip + 1;
      state[ip_register] = ip;

      counter = counter + 1;
      if ip >= opcodes.len() { break; }
      // if counter > 7484592 { break; }
    }     
    println!("Day 19 part 2 {:?}", state[0]);
    // 10551368 too low
    return state[0].try_into().unwrap();
}
