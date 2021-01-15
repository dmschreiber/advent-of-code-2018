#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle13_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(20==super::solve("./inputs/puzzle13-test.txt".to_string()));
    assert!(20==super::solve("./inputs/puzzle13-test2.txt".to_string()));
  }

  #[test]
  pub fn puzzle13_prod() {
    assert!(common::format_binary(10)=="1010");
    assert!(108+60+92+42==super::solve("./inputs/puzzle13.txt".to_string()));
  }
}

use crate::common;
use std::convert::TryInto;
use std::collections::HashMap;


#[derive(PartialEq,Clone,Debug)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(PartialEq,Clone,Debug)]
pub enum Turn {
  Left,
  Straight,
  Right,
}
// left, straight, right
#[derive(PartialEq,Clone,Debug)]
pub struct Cart {
  cart_id : usize,
  position : (usize,usize),
  direction : Direction,
  intersection : Turn,
  crashed : bool,
}

fn next_turn (turn : Turn) -> Turn {
  match turn {
    Turn::Left => Turn::Straight,
    Turn::Straight => Turn::Right,
    Turn::Right => Turn::Left,
  }
}
pub fn solve_part1(map : &HashMap<(usize,usize),char>, carts : &mut Vec<Cart>, part2 : bool) -> (usize,usize) {
  let mut tick = 1;
  loop {
    move_carts(map, carts);
    // println!("Tick {} carts left {:?}", tick,carts.iter().filter(|my_cart| my_cart.crashed == false).map(|my_cart| my_cart.cart_id).collect::<Vec<usize>>());
    tick = tick + 1;
    if carts.iter().filter(|my_cart| my_cart.crashed == false).count() == 0 { return carts[0].position; }
    if carts.iter().filter(|my_cart| my_cart.crashed == false).count() == 1 { 
      println!("only one remains!");
      return carts.iter().filter(|my_cart| my_cart.crashed == false).map(|my_cart| my_cart.position).collect::<Vec<(usize,usize)>>()[0]; 
    }
    if !part2 && carts.iter().filter(|my_cart| my_cart.crashed ).count() == 2 { 
      return carts.iter().filter(|my_cart| my_cart.crashed == true).map(|my_cart| my_cart.position).collect::<Vec<(usize,usize)>>()[0]; 
    }

  }

}

fn get_direction(dir : Direction, turn : Turn) -> Direction {
  match turn {
    Turn::Left if dir == Direction::North => Direction::West,
    Turn::Left if dir == Direction::East => Direction::North,
    Turn::Left if dir == Direction::South => Direction::East,
    Turn::Left if dir == Direction::West => Direction::South,

    Turn::Right if dir == Direction::North => Direction::East,
    Turn::Right if dir == Direction::West => Direction::North,
    Turn::Right if dir == Direction::South => Direction::West,
    Turn::Right if dir == Direction::East => Direction::South,

    Turn::Straight => dir,
    _ => { panic!("uncovered"); }
  }
}

fn move_carts(map : &HashMap<(usize,usize),char>, carts : &mut Vec<Cart>) {

  carts.sort_by_key(|k| k.position.0*100000+k.position.1);
  // println!("{:?}", carts.iter().map(|c| c.position).collect::<Vec<(usize,usize)>>());
  let clone_carts = carts.clone();
  for c in clone_carts.iter().filter(|cart| cart.crashed == false) {
    // println!("working on cart at {:?}", c.position);
    let mut new_position = c.position;
    match c.direction {
      Direction::East => { new_position.1 = new_position.1+1; },
      Direction::South => { new_position.0 = new_position.0+1; },
      Direction::West => { new_position.1 = new_position.1-1; },
      Direction::North  => { new_position.0 = new_position.0-1; },
      // _ => { panic!("uncovered"); }
    }
    let map_char = *map.get(&new_position).unwrap();
    let mut new_direction = c.direction.clone();
    let mut new_intersection = c.intersection.clone();
    match c.direction {
      Direction::East if map_char == '-' => {  }
      Direction::East if map_char == '\\' => {new_direction = Direction::South; }
      Direction::East if map_char == '/' => { new_direction = Direction::North; }
      Direction::South if map_char == '|' => {  }
      Direction::South if map_char == '/' => { new_direction = Direction::West; }
      Direction::South if map_char == '\\' => {new_direction = Direction::East; }
      Direction::West if map_char == '-' => {  }
      Direction::West if map_char == '\\' => { new_direction = Direction::North; }
      Direction::West if map_char == '/' => { new_direction = Direction::South; }
      Direction::North if map_char == '|' => {  }
      Direction::North if map_char == '/' => { new_direction = Direction::East; }
      Direction::North if map_char == '\\' => { new_direction = Direction::West; }
      Direction::North | Direction::East | Direction::South | Direction::West if map_char == '+' => {
        new_direction = get_direction(c.direction.clone(), c.intersection.clone()); new_intersection = next_turn(c.intersection.clone());
      },
      _ => { panic!("uncovered {}", map_char); }
    }
    let which_cart = carts.iter().position(|my_cart| my_cart.cart_id == c.cart_id).unwrap();
    if let Some(change_cart) = carts.get_mut(which_cart) {
      change_cart.direction = new_direction;
      change_cart.position = new_position;
      change_cart.intersection = new_intersection;
  
    }

    if carts.iter().filter(|find_cart| find_cart.position == new_position && !find_cart.crashed && find_cart.cart_id != c.cart_id).count() > 0 {
      let other_cart_id = carts.iter()
                      .filter(|find_cart| find_cart.position == new_position && !find_cart.crashed && find_cart.cart_id != c.cart_id)
                      .map(|find_cart| find_cart.cart_id)
                      .collect::<Vec<usize>>()[0];
      // println!("CRASH {:?} with {:?}", c.cart_id, carts.iter().filter(|find_cart| find_cart.position == new_position && !find_cart.crashed && find_cart.cart_id != c.cart_id).map(|my_cart| my_cart.cart_id).collect::<Vec<usize>>());
      let which_cart = carts.iter().position(|my_cart| my_cart.cart_id == c.cart_id).unwrap();
      if let Some(change_cart) = carts.get_mut(which_cart) {
        change_cart.crashed = true;
      }
      let which_cart = carts.iter().position(|my_cart| my_cart.cart_id == other_cart_id).unwrap();
      if let Some(change_cart) = carts.get_mut(which_cart) {
        change_cart.crashed = true;
      }
    }
  }

}

fn load_structures (lines : &Vec<String>) -> (Vec<Cart>, HashMap<(usize,usize),char>) {

  let mut map = HashMap::new();

  let mut carts = vec![];
  let mut cart_id = 0;
  for (row,l) in lines.iter().enumerate() {
    for (col,c) in l.as_bytes().iter().enumerate() {
      match c {
        b'-' | b'|' | b'/' | b'\\' | b'+' => { map.insert((row,col),*c as char); }
        b'>'  => { cart_id += 1; carts.push(Cart{ cart_id : cart_id, position : (row,col), direction : Direction::East, intersection : Turn::Left, crashed : false } ); map.insert((row,col), '-'); }
        b'<'  => { cart_id += 1; carts.push(Cart{ cart_id : cart_id, position : (row,col), direction : Direction::West, intersection : Turn::Left, crashed : false } ); map.insert((row,col), '-'); }
        b'^'  => { cart_id += 1; carts.push(Cart{ cart_id : cart_id, position : (row,col), direction : Direction::North, intersection : Turn::Left, crashed : false } ); map.insert((row,col), '|'); }
        b'v'  => { cart_id += 1; carts.push(Cart{ cart_id : cart_id, position : (row,col), direction : Direction::South, intersection : Turn::Left, crashed : false } ); map.insert((row,col), '|'); }
        _ => { // nothing }
        }
      }
    }
  }
  return (carts,map);
}


pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let (mut carts, map) = load_structures(&lines);
  let result = solve_part1(&map, &mut carts, false);
  println!("Day 13 part 1 is {},{}", result.1, result.0);
  let part1 = result.0 + result.1;

  let (mut carts, map) = load_structures(&lines);
  let result = solve_part1(&map, &mut carts, true);

  println!("Day 13 part 2 is {},{}", result.1, result.0);
  return (part1 + result.0+result.1).try_into().unwrap();
}
