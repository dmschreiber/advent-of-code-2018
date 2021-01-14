#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle10_test() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle10-test.txt".to_string());
    println!("{:?}", super::parse_line("position=<-3,  6> velocity=< 2, -1>".to_string()));
    assert!(super::parse_line("position=<-3,  6> velocity=< 2, -1>".to_string()) == ((-3,6),(2,-1)));
  }

  #[test]
  pub fn puzzle10_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle10.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;
use std::collections::HashMap;

// position=<-3,  6> velocity=< 2, -1>
lazy_static! {
  static ref NUMBER_REGEX: Regex = Regex::new(r"^position=<\s*(-?[0-9+]+),\s*(-?[0-9+]+)> velocity=<\s*(-?[0-9+]+),\s*(-?[0-9+]+)>$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn parse_line(expression : String) -> ((i64,i64),(i64,i64)) {

  if let Some(inner) = NUMBER_REGEX.captures(&expression) {
    return ((inner[1].parse::<i64>().unwrap(),inner[2].parse::<i64>().unwrap()),(inner[3].parse::<i64>().unwrap(),inner[4].parse::<i64>().unwrap()));
  }
  panic!("not a number");
}

#[derive(Debug,Clone)]
pub struct Particle {
  position : (i64,i64),
  velocity : (i64,i64),
}

fn detect_pattern(world : &HashMap<(i64,i64),Vec<Particle>>) -> bool {
  let mut vert = false;
  let mut horiz = false;

  for each_point in world.keys() {
    if world.keys().filter(|(my_x,my_y)| *my_x == each_point.0 && (my_y - each_point.1).abs() <= 3).count() >= 7 {
      println!("vert detect {:?}", each_point);
      vert = true;
    }
    // if each_point.0 == 7 && each_point.1 == 0 {
    //   println!("{:?}",world.keys().filter(|(my_x,my_y)| *my_y == each_point.1 && (my_x - each_point.0).abs() <= 1).map(|a| *a).collect::<Vec<(i64,i64)>>());
    // }

    if world.keys().filter(|(my_x,my_y)| *my_y == each_point.1 && (my_x - each_point.0).abs() <= 1).count() >= 3 {
      println!("horiz detect {:?}", each_point);
      horiz = true;
    }
  }
  return vert && horiz;
}

fn print_world (world : &HashMap<(i64,i64),Vec<Particle>>) {
  let min_x : i64 = world.keys().map(|p| p.0).min().unwrap();
  let max_x : i64 = world.keys().map(|p| p.0).max().unwrap();
  let min_y : i64 = world.keys().map(|p| p.1).min().unwrap();
  let max_y : i64 = world.keys().map(|p| p.1).max().unwrap();

  println!("x: {}-{}, y: {}-{}", min_x,max_x,min_y,max_y);
  let mut count = 0;
  for y in min_y..=max_y {
    for x in min_x-2..=max_x+2 {
      if let Some(_p) = world.get(&(x,y)) {
        print!("#");
        count = count + 1;
      } else {
        print!(".");
      }
    }
    println!();
  }
  println!("Count is {}", count);
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");

  let mut world : HashMap<(i64,i64),Vec<Particle>> = HashMap::new();
  for l in lines {
    let (position,velocity) = parse_line(l);
    world.insert(position,vec![Particle{ position : position, velocity : velocity}]);

  }
  
  let mut new_world : HashMap<(i64,i64),Vec<Particle>> ;
  // print_world(&world);

  let mut seconds = 0;
  while !detect_pattern(&world) {
    new_world = HashMap::new();
    // let all_pos : Vec<(i64,i64)> = world.keys().map(|p| *p).collect();;
    for p_v in world.values() {
      for p in p_v {
        let new_x = p.position.0 + p.velocity.0;
        let new_y = p.position.1+p.velocity.1;
        if let Some(new_p) = new_world.get_mut(&(new_x,new_y)) {
          new_p.push(Particle{ position : (new_x,new_y), velocity : p.velocity });
        } else {
          new_world.insert((new_x,new_y),vec![Particle{ position : (new_x,new_y), velocity : p.velocity }]);
        }
      }
    }
    world = new_world.clone();
    seconds = seconds + 1;
  }
  print_world(&world);
  println!("seconds {}", seconds);
  return seconds.try_into().unwrap();
}
