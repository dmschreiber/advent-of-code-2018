#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  #[test]
  pub fn puzzle22_test() {
    let mut map = HashMap::new();
    assert!(1==super::erosion_level(1,0,510,&mut map)%3);
    assert!(0==super::erosion_level(0,1, 510,&mut map)%3);
    assert!(114==super::solve("510,10,10".to_string()));
  }

  #[test]
  pub fn puzzle22_prod() {
    super::solve("3558,15,740".to_string());
  }
}

use std::convert::TryInto;
use std::collections::HashMap;
pub extern crate pathfinding;
use pathfinding::prelude::astar;

fn erosion_level(x : u32, y : u32, depth : u32, map : &mut HashMap<(u32,u32),u32>) -> u32 {
  return (geo_index(x,y, depth,map) + depth ) % 20183;
}

fn geo_index (x : u32, y : u32, depth : u32, map : &mut HashMap<(u32,u32),u32>) -> u32 {
  if let Some(r) = map.get(&(x,y)) {
    return *r;
  } else {
    let result;
    if x==0 && y == 0 { map.insert((0,0),0); return 0; }
    else if x == 0 { result = y * 48271; map.insert((x,y),result); return result; }
    else if y == 0 { result = x * 16807; map.insert((x,y),result); return result; }
    else { 
      result = erosion_level(x-1, y, depth, map)*erosion_level(x, y-1, depth, map); 
      map.insert((x,y),result); 
      return result; }
  }
}

fn get_neighbors(spot : &Pos, depth : u32, map : &mut HashMap<(u32,u32),u32>) -> Vec<(Pos,isize)> {

  let n : Vec<(isize,isize)>= vec![(-1,0), (1,0), (0,-1), (0,1)];

  let curr_type = get_type(spot.x,spot.y,depth,map);
  let pos_n = n.iter().map(|p| (p.0+spot.x as isize,p.1+spot.y as isize))
              .filter(|(p_x,p_y)| *p_x >= 0 && *p_y >= 0).map(|(x,y)| (x as u32,y as u32)).collect::<Vec<(u32,u32)>>();


  let other_gear = get_gear(curr_type).iter().filter(|g| **g != spot.gear).map(|g| *g).collect::<Vec<Gear>>();
  let mut retval = other_gear.iter().map(|g| Pos{ x: spot.x, y: spot.y, gear: *g}).map(|p| (p,7)).collect::<Vec<(Pos,isize)>>();

  for neighbor in pos_n {
    if get_gear(get_type(neighbor.0,neighbor.1,depth,map)).contains(&spot.gear) {
      retval.push( (Pos { x: neighbor.0, y: neighbor.1, gear: spot.gear },1));
    }
  }
  return retval;
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum Gear {
  Torch,
  Climbing,
  Neither
}
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Pos {
  x : u32, 
  y : u32,
  gear : Gear,
}

fn get_gear(my_type : u32) -> Vec<Gear> {
  match my_type {
    0 => vec![Gear::Climbing, Gear::Torch],
    1 => vec![Gear::Climbing, Gear::Neither],
    2 => vec![Gear::Torch, Gear::Neither],
    _ => { panic!("invalid type"); }
  }
}

fn get_type(x : u32, y : u32, depth : u32, map : &mut HashMap<(u32,u32),u32>) -> u32 {
  return erosion_level(x, y, depth, map) % 3;
  // 0 - rocky - Climbing || Torch (1:Climbing, 2:Torch)
  // 1 - wet - Climing || Neither (0:Climbing, 2:Neither)
  // 2 - narrow - Torth || Neither (0:Torch, 1:Neither)
}

fn print(end : Pos, path : Vec<Pos>, depth : u32, map : &mut HashMap<(u32,u32),u32>) {
  let mut path_map = HashMap::new();
  for p in path {
    path_map.insert((p.x,p.y),p);
  }

  for y in 0..=end.y+5 {
    for x in 0..=end.x+5 {
      let t = get_type(x,y,depth,map);
      if x==0 && y==0 { 
        print!("M"); 
      } else {
        if let Some(p) = path_map.get(&(x,y)) {
          match p.gear {
            Gear::Climbing => { print!("C"); }
            Gear::Torch => { print!("T"); }
            Gear::Neither => { print!("N"); }
          }
        } else {
          match t {
            0 => { print!(".");}
            1 => { print!("=");}
            2 => { print!("|");}
            _ => { panic!("invalid type"); }
          }
        }

      }
      
    }
    println!();
  }
}

pub fn solve(input : String) -> i64 {
  let args = input.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

  let depth = args[0];
  let target = (args[1],args[2]);
  let mut map = HashMap::new();

  let mut sum = 0;
  for x in 0..=target.0 {
    for y in 0..=target.1 {
      // println!("({},{}) - {}", x,y, erosion_level(x,y,depth,&mut map));
      if x == target.0 && y == target.1 { // nothing 
      } else { 
        sum = sum + get_type(x,y,depth,&mut map);
      }
    }
  }

  println!("Sum is {}", sum);

  let start = Pos { x: 0, y: 0, gear : Gear::Torch };
  let end = Pos { x: target.0, y: target.1, gear: Gear::Torch };

  println!("{:?}", get_neighbors(&Pos{x: 4, y: 1, gear: Gear::Neither}, depth, &mut map));
  println!("{:?}", get_neighbors(&Pos{x: 4, y: 1, gear: Gear::Climbing}, depth, &mut map));
  println!();
  let result : Option<(Vec<Pos>,isize)> = 
  astar(&start, 
    |p| get_neighbors(p, depth,&mut map), 
    |p| (end.x as isize-p.x as isize).abs() + (end.y as isize -p.y as isize).abs(),
    |p| *p == end);

  println!("{:?}", result);
  print(end, result.unwrap().0,depth, &mut map);

  return sum.try_into().unwrap();
}
