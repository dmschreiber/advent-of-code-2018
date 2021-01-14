#[cfg(test)]
mod tests {
  use crate::common;
  use std::convert::TryInto;

  #[test]
  pub fn puzzle11_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(4==super::calculate_power(3,5,8));
    assert!(-5==super::calculate_power(122,79,57));
    assert!(0==super::calculate_power(217,196,39));
    assert!(super::solve("18".to_string())==29);

    println!("calculate 9 - {} vs {}", super::calculate_power_grid(1, 1, 18, 9),     super::experimental_power_grid(1,1,18,9)  );
    assert!(super::solve_part2("18".to_string())==113);
  }

  #[test]
  pub fn puzzle11_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("2187".to_string());
    super::solve_part2("2187".to_string());
  }
}

use std::convert::TryInto;
use std::collections::HashMap;
use cached::proc_macro::cached;
use cached::SizedCache;

fn calculate_power (x : i64, y : i64, serial_number : i64) -> i64 {
  let rack_id = x + 10;
  let mut power = rack_id * y;
  power = power + serial_number;
  power = power * rack_id;
  power = (power / 100) % 10;
  power = power - 5;
  return power;
}

fn factor(num: i32) -> Vec<i32> {
  let mut factors: Vec<i32> = Vec::new(); // creates a new vector for the factors of the number

  for i in 1..((num as f32).sqrt() as i32 + 1) { 
      if num % i == 0 {
          factors.push(i); // pushes smallest factor to factors
          factors.push(num/i); // pushes largest factor to factors
      }
  }
  factors.sort(); // sorts the factors into numerical order for viewing purposes
  factors // returns the factors
}

// #[cached(
//   type = "SizedCache<i64, i64>",
//   create = "{ SizedCache::with_size(100000) }",
//   convert = r#"{ x*1000000000+y*1000000+size*1000+serial_number*1000000000000 }"#
// )]
fn experimental_power_grid(x : i64, y : i64, serial_number : i64, size : i64 ) -> i64 {
  // println!("Trying {},{} - {}", x, y, size);
  let f = factor(size.try_into().unwrap());
  if f.len() == 2 { return calculate_power_grid(x, y, serial_number,size); }
  else {
    let f1 = f[1];

    return (0..size).step_by(f1.try_into().unwrap()).map(|i| (0..size).step_by(f1.try_into().unwrap()).fold(0, |acc,j| acc + experimental_power_grid(x+i,y+j,serial_number,f1.try_into().unwrap()))).sum::<i64>();
  }
}

fn calculate_power_grid(x : i64, y : i64, serial_number : i64, size : i64 ) -> i64 {
    let power =     (0..size).map(|i| (0..size).fold(0, |acc,j| acc + calculate_power(x+i,y+j,serial_number))).sum::<i64>();
    return power;
}

pub fn solve_part2(serial_string : String) -> i64 {

  let serial_number = serial_string.parse::<i64>().unwrap();

  let mut power_map = HashMap::new();
  for x in 1..=300 {
    for y in 1..=300 {
      let mut power = calculate_power(x,y,serial_number);
      if let Some(i1) = power_map.get(&(x-1,y)) {
        power = power + i1;
      }
      if let Some(i2) = power_map.get(&(x,y-1)) {
        power = power + i2;
      }
      if let Some(i3) = power_map.get(&(x-1,y-1)) {
        power = power - i3;
      }
      power_map.insert((x,y),power);
    }
  }

  let mut largest = 0; // *power_map.values().max().unwrap();
  let mut largest_point = (0,0); // power_map.keys().filter(|k| *power_map.get(k).unwrap() == largest).map(|point| *point).collect::<Vec<(i64,i64)>>()[0];
  let mut largest_z = 1;
  for z in 1..=300 {
    println!("Trying {} - largest is {} - {:?}", z, largest, largest_point);
    for x in 1..=300-z-1 {
      for y in 1..=300-z-1 {
        let power = power_map.get(&(x,y)).unwrap() + power_map.get(&(x+z,y+z)).unwrap() - power_map.get(&(x,y+z)).unwrap() - power_map.get(&(x+z,y)).unwrap();
        if power > largest {
          largest = power;
          largest_point = (x+1,y+1);
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


  let mut largest = 0; 
  let mut largest_point = (0,0); 
  let z = 3;
  println!("Trying {} - largest is {}", z, largest);
  for x in 1..=300-z-1 {
    for y in 1..=300-z-1 {
      let power = calculate_power_grid(x, y, serial_number, z);

      // let power : i64 = power_map.keys().filter(|(my_x,my_y)| *my_x >= x && *my_x <= x+z-1 && *my_y >= y && *my_y <= y+z-1).map(|k| power_map.get(&k).unwrap()).sum();
      if power > largest {
        largest = power;
        largest_point = (x,y);
      }
    }
  }

  println!("Largest point is {:?}, with power {}", largest_point, largest);
  return largest.try_into().unwrap();
}
