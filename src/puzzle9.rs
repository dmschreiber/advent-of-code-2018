#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle9_test() {
    assert!(common::format_binary(10)=="1010");
    assert!((10,1618)==super::get_number_between_text("10 players; last marble is worth 1618 points".to_string()));
    assert!(32==super::solve("9 players; last marble is worth 25 points".to_string()));
    assert!(8317==super::solve("10 players; last marble is worth 1618 points".to_string()));
    assert!(37305==super::solve("30 players; last marble is worth 5807 points".to_string()));
  }

  #[test]
  pub fn puzzle9_prod() {
    assert!(common::format_binary(10)=="1010");
    assert!(398502 == super::get_high_score(428, 70825));
    assert!(3352920421 == super::get_high_score(428, 100*70825));
  }
}

use std::convert::TryInto;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Marble {
  num : u32,
  clockwise : u32,
  counter_clockwise : u32,
}

lazy_static! {
  static ref NUMBERS_REGEX: Regex = Regex::new(r"^([0-9+]+) players; last marble is worth ([0-9+]+) points$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn get_number_between_text(expression : String) -> (u32,u32) {

  if let Some(inner) = NUMBERS_REGEX.captures(&expression) {
    return (inner[1].parse::<u32>().unwrap(),inner[2].parse::<u32>().unwrap());
  }
  panic!("not a number");
}

fn get_marble_clockwise(map : &HashMap<u32,Marble>, which_marble : u32, offset : u32) -> u32 {
  let m = map.get(&which_marble).unwrap();

  let mut current = m;
  for _i in 0..offset {
    current = map.get(&current.clockwise).unwrap();
  }
  return current.num;
}

fn get_marble_counter_clockwise(map : &HashMap<u32,Marble>, which_marble : u32, offset : u32) -> u32 {
  let m = map.get(&which_marble).unwrap();

  let mut current = m;
  for _i in 0..offset {
    current = map.get(&current.counter_clockwise).unwrap();
  }
  return current.num;
}

fn put_marble_after(map : &mut HashMap<u32,Marble>, to_put : u32, after : u32) {
  let target = map.get_mut(&after).unwrap();
  let next = target.clockwise;
  target.clockwise = to_put;

  let next_target = map.get_mut(&next).unwrap();
  next_target.counter_clockwise = to_put;

  let m = Marble{ num : to_put, clockwise : next, counter_clockwise : after };
  map.insert(to_put,m);

}

fn remove_marble(map : &mut HashMap<u32,Marble>, which : u32) -> u32 {
  let target = map.get(&which).unwrap().clone();
  let next_marble = target.clockwise;
  let prev_marble = target.counter_clockwise;

  let mut prev = map.get_mut(&prev_marble).unwrap();
  prev.clockwise = next_marble;
  let mut next = map.get_mut(&next_marble).unwrap();
  next.counter_clockwise = prev_marble;

  map.remove(&which);
  return next_marble;
}

#[allow(dead_code)]
fn print_map(map : &HashMap<u32,Marble>, current_marble : u32) {
  let mut index = 0;
  let mut target = map.get(&index).unwrap();
  // print!("{}", index);

  loop {
    if target.num == current_marble {
      print!("({})", target.num);
    } else {
      print!("{}", target.num);
    }
    index = target.clockwise;
    let prev = target.num;

    if target.clockwise == 0 { break; }
    print!(",");
    target = map.get(&index).unwrap();
    if target.counter_clockwise != prev { panic!("{} has previous {} v counter {}", target.num, prev, target.counter_clockwise); }
  }
  println!();

}

pub fn get_high_score(players : u32, last_marble : u32) -> i64 {
  let high_score;
  
  let mut current_player = 1;
  let mut current_marble = 1;
  let mut scores = HashMap::new();

  let mut marble_map : HashMap<u32,Marble> = HashMap::new();

  marble_map.insert(0,Marble{ num: 0, clockwise : 1, counter_clockwise: 1});
  marble_map.insert(1,Marble{ num: 1, clockwise : 0, counter_clockwise: 0});

  // marbles = vec![0,1];
  for m in 2..=last_marble {
    if m % 23 == 0 {
      if scores.get(&current_player) == None {
        scores.insert(current_player,0);
      }

      if let Some(score) = scores.get_mut(&current_player) {
        let additional_m  = get_marble_counter_clockwise(&marble_map, current_marble, 7);
        current_marble = remove_marble(&mut marble_map, additional_m);

        *score = *score + m + additional_m;
      } 
    } else {
      let target = get_marble_clockwise(&marble_map, current_marble, 1);
      put_marble_after(&mut marble_map,m,target);
      current_marble = m;
  
    }
    // print_map(&marble_map, current_marble);
    // println!("[{}] ({}) {:?}", current_player+1, marbles[current_marble_index], marbles);
    current_player = (current_player + 1) % players;
    // if m % 100000 == 0 { println!("PRogress {} of {} - {:?}", m, last_marble, *scores.values().max().unwrap()); }
  }

  high_score = *scores.values().max().unwrap();
  println!("{:?}", high_score);
  return high_score.try_into().unwrap();  
}
pub fn solve(input : String) -> i64 {

  let (players,last_marble) = get_number_between_text(input);
  let part1 = get_high_score(players, last_marble);
  println!("Part 1 {}", part1);

  let part2 = get_high_score(players, 100*last_marble);
  println!("Part 2 {}", part2);
  // 
  //          3,352,920,421

  return part1;

}
