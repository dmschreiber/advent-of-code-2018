#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle12_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(super::get_init_state(&"initial state: #...##.#...#..#.#####.##.#..###.#.#.###....#...#...####.#....##..##..#..#..#..#.#..##.####.#.#.###".to_string()) == "#...##.#...#..#.#####.##.#..###.#.#.###....#...#...####.#....##..##..#..#..#..#.#..##.####.#.#.###".to_string());
    assert!(super::get_mapping(&"..#.. => #".to_string())==("..#..".to_string(),'#'));
    super::solve("./inputs/puzzle12-test.txt".to_string());
  }

  #[test]
  pub fn puzzle12_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle12.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
  static ref MAPPING_REGEX: Regex = Regex::new(r"^(.+) => (.)$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn get_mapping(expression : &String) -> (String,char) {

  if let Some(inner) = MAPPING_REGEX.captures(&expression) {
    let left = inner[1].to_string();
    let right = inner[2].as_bytes()[0] as char;
    return (left,right);
  }
  panic!("not valid");
}

fn get_init_state(expression : &String) -> String {
  let initial_state = expression[15..].to_string();
  return initial_state.to_string();
}

fn match_mapping(map : &HashMap<isize,char>, mapping : &(String,char), which_pot : isize) -> bool {

  for (rel_pos,i) in (which_pot-2..=which_pot+2).enumerate() {
    let mut pot_plant = '.';
    if let Some(pot) = map.get(&i) {
      pot_plant = *pot;
    } 
    if mapping.0.as_bytes()[rel_pos] as char != pot_plant {
      return false;
    } 
  }
  return true;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let initial_state = get_init_state(&lines[0]);
  let mut pot_map = HashMap::new();
  for (location,b) in initial_state.as_bytes().iter().enumerate() {
    pot_map.insert(location as isize,*b as char);
  }

  let mut mappings = vec![];
  for l in lines[2..].iter() {
    let map = get_mapping(&l);
    mappings.push(map);
  }
  println!("{:?}", mappings);
  let mut new_pot_map;

  for i in 0..20 {
    new_pot_map = HashMap::new();
    for pot_number in *pot_map.keys().min().unwrap()-2..=*pot_map.keys().max().unwrap()+2 {
      for mapping in &mappings {
        if match_mapping(&pot_map, mapping, pot_number) {
          println!("pot {} matched result is {} ({})", pot_number, mapping.1, mapping.0);
          new_pot_map.insert(pot_number,mapping.1);
        } 
      }
      if new_pot_map.get(&pot_number) == None {
        println!("pot {} no match so .", pot_number);
        new_pot_map.insert(pot_number,'.');
      }      
    }

    pot_map = new_pot_map.clone();

    let mut pots : Vec<isize>= pot_map.keys().map(|k| *k).collect();
    pots.sort();
    println!("{}", pots.iter().map(|k| pot_map.get(&k).unwrap()).collect::<String>());
  }

  let sum = pot_map.keys().filter(|k| *pot_map.get(&k).unwrap() == '#').map(|k| *k).sum::<isize>();
  println!("sum is {}", sum);
  return sum.try_into().unwrap();
}
