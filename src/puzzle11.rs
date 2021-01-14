#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle11_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(4==super::calculate_power(3,5,8));
    assert!(-5==super::calculate_power(122,79,57));
    assert!(0==super::calculate_power(217,196,39));
    assert!(super::solve("18".to_string())==29);
    assert!(super::solve_part2("18".to_string())==113);
  }

  #[test]
  pub fn puzzle11_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("2187".to_string());
    super::solve_part2("2187".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use std::collections::HashMap;

fn calculate_power (x : i64, y : i64, serial_number : i64) -> i64 {
  let rack_id = x + 10;
  let mut power = rack_id * y;
  power = power + serial_number;
  power = power * rack_id;
  power = (power / 100) % 10;
  power = power - 5;
  return power;
}

fn calculate_power_grid(x : i64, y : i64, serial_number : i64, size : i64 ) -> i64 {
  let mut power = 0;
  for i in 0..size {
    for j in 0..size {
      power = power + calculate_power(x+i, y+j, serial_number);
    }
  }
  return power;
}

pub fn solve_part2(serial_string : String) -> i64 {

  let serial_number = serial_string.parse::<i64>().unwrap();
  // let mut power_map = HashMap::new();
  // for x in 1..=300 {
  //   for y in 1..=300 {
  //     power_map.insert((x,y),calculate_power(x, y, serial_number));
  //   }
  // }


let mut largest = 0; // *power_map.values().max().unwrap();
let mut largest_point = (0,0); // power_map.keys().filter(|k| *power_map.get(k).unwrap() == largest).map(|point| *point).collect::<Vec<(i64,i64)>>()[0];
let mut largest_z = 1;
for z in (1..=300).rev() {
  println!("Trying {} - largest is {}", z, largest);
  for x in 1..=300-z-1 {
    for y in 1..=300-z-1 {
      let power = calculate_power_grid(x, y, serial_number, z);
//      let power : i64 = power_map.keys().filter(|(my_x,my_y)| *my_x >= x && *my_x <= x+z-1 && *my_y >= y && *my_y <= y+z-1).map(|k| power_map.get(&k).unwrap()).sum();
      if power > largest {
        largest = power;
        largest_point = (x,y);
        largest_z = z;
      }
    }
  }
}
  println!("Largest point is {:?}, {} with power {}", largest_point, largest_z, largest);
  return largest.try_into().unwrap();
}




pub fn solve(serial_string : String) -> i64 {

  let serial_number = serial_string.parse::<i64>().unwrap();
  // let mut power_map = HashMap::new();
  // for x in 1..=300 {
  //   for y in 1..=300 {
  //     power_map.insert((x,y),calculate_power(x, y, serial_number));
  //   }
  // }


let mut largest = 0; 
let mut largest_point = (0,0); 
let mut largest_z = 3;
let z = 3;
println!("Trying {} - largest is {}", z, largest);
  for x in 1..=300-z-1 {
    for y in 1..=300-z-1 {
      let power = calculate_power_grid(x, y, serial_number, z);

      // let power : i64 = power_map.keys().filter(|(my_x,my_y)| *my_x >= x && *my_x <= x+z-1 && *my_y >= y && *my_y <= y+z-1).map(|k| power_map.get(&k).unwrap()).sum();
      if power > largest {
        largest = power;
        largest_point = (x,y);
        largest_z = z;
      }
    }
  }

  println!("Largest point is {:?}, with power {}", largest_point, largest);
  return largest.try_into().unwrap();
}
