#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle21_test() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle21-test.txt".to_string());
  }

  #[test]
  pub fn puzzle21_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle21.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use crate::puzzle16;
use crate::puzzle19;

pub fn run (opcodes : &Vec<Vec<usize>>, initial_state : Vec<usize>, ip_register : usize) -> Vec<usize> {
  let mut ip = 0;
  let mut state = initial_state.clone();

  let mut history = vec![];

  loop {
    let o = &opcodes[ip];

    state[ip_register] = ip;
    state = puzzle16::op_code(o[0], o[1..].to_vec(), &state);
    ip = state[ip_register];
    ip = ip + 1;
    state[ip_register] = ip;

    let key = format!("{:?}", state);
    if history.contains(&key) {
      panic!("infinite loop");
    } else {
      if ip == 28 {
        println!("{}", key);
      }
      history.push(key);
    }
    if ip >= opcodes.len() {break; }
  }
  return state;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  let ip_register;

  if let Some(parse_ip) = puzzle19::IP_REGEX.captures(&lines[0]) {
    ip_register = parse_ip[1].parse::<usize>().unwrap();
    println!("IP register {}", ip_register);
  } else {
    panic!("parse failure {}", lines[0]);
  }


  let mut opcodes = vec![];
  for l in lines[1..].iter() {
    if let Some(parse_opcode) = puzzle19::OPCODE_REGEX.captures(&l) {
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


  let mut history = vec![];
  let mut r1 = 0;
  let mut r2;
  let mut r5;
  loop {
    r5 = r1 | 65536;
    r1 = 8586263;
    loop {
      r2 = r5 & 255;
      r1 = r1 + r2;
      r1 = 16777215 & r1;
      r1 = 65899 * r1;
      r1 = 16777215 & r1;
    
      if r5 >= 256 {
        r5 = r5 / 256;
      } else {
        break;
      }
    }

    if history.contains(&r1) {
      println!("first one was {}, last one was {}", history.first().unwrap(), history.last().unwrap());      
      break;
    } else {
      history.push(r1); 
    }
  }
    // rewritten above ^^

    // let mut state = vec![0; 6];
    // state = run(&opcodes,state,ip_register);

  let retval = *history.last().unwrap();
  return retval.try_into().unwrap();
}
