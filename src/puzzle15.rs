#[cfg(test)]
mod tests {
  use crate::common;
  use std::time::Instant;
  #[test]
  pub fn puzzle15_test() {
    assert!(common::format_binary(10)=="1010");
    let start = Instant::now();
    assert!(18740==super::solve("./inputs/puzzle15-test.txt".to_string()));
    assert!(1140==super::solve_part2("./inputs/puzzle15-test.txt".to_string()));
    println!("Complete in {:?}", start.elapsed());
  }

  #[test]
  pub fn puzzle15_prod() {
    assert!(common::format_binary(10)=="1010");
    let start = Instant::now();
    assert!(250648==super::solve("./inputs/puzzle15.txt".to_string()));
    println!("Part 1 complete in {:?}", start.elapsed());
    assert!(42224==super::solve_part2("./inputs/puzzle15.txt".to_string()));
    println!("Part 2 complete in {:?}", start.elapsed());
  }
}

use crate::common;
use std::convert::TryInto;
use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq)]
pub enum UnitType {
  Goblin,
  Elf,
  None,
}

#[derive(Debug,Clone)]
pub struct Unit {
  kind : UnitType,
  position : (isize,isize),
  hit_points : i32,
}

fn get_neighbors_with_units(map : &HashMap<(isize,isize),char>, p : &(isize,isize)) -> Vec<(isize,isize)> {
  let v = vec![(-1,0), (0,-1), (0,1), (1,0)];

  v.iter().map(|n| (p.0+n.0,p.1+n.1)).filter(|n| map.get(n) != None && *map.get(n).unwrap() == '.').collect::<Vec<(isize,isize)>>().to_vec()
}

fn get_neighbors(map : &HashMap<(isize,isize),char>, units : &Vec<Unit>, p : &(isize,isize)) -> Vec<(isize,isize)> {
  let v = vec![(-1,0), (0,-1), (0,1), (1,0)];
  let n = v.iter().map(|n| (p.0+n.0,p.1+n.1));
  let mut filtered_n = vec![];

  for neighbor in n {
    if let Some(p) = map.get(&neighbor) {
      if *p == '.' && units.iter().filter(|u| u.position == neighbor && u.hit_points > 0).count() == 0 {
        filtered_n.push (neighbor);
      }
    }
  }

  filtered_n
}

fn manhattan_distance (p1 : (isize,isize), p2 : (isize,isize)) -> isize {
  return (p1.0-p2.0).abs()+(p1.1-p2.1).abs();
}

pub extern crate pathfinding;

use pathfinding::prelude::astar;



fn other_path(map : &HashMap<(isize,isize),char>, 
          point_a : (isize,isize), 
          point_b : (isize,isize), 
          units : &Vec<Unit>) -> Option<(std::vec::Vec<(isize, isize)>, isize)> {

  let result : Option<(Vec<(isize,isize)>,isize)> = 
                    astar(&point_a, 
                    |p| get_neighbors(map, units, p).into_iter().map(|p| (p, 1)).collect::<Vec<((isize,isize),isize)>>(), 
                    |p| manhattan_distance(*p, point_b),
                    |p| *p == point_b);
  return result;
}

pub fn print_map_history(map : &HashMap<(isize,isize),char>, units : &Vec<Unit>, history : &Vec<(isize,isize)>) {
  let max_row = *map.keys().map(|(r,_c)| r).max().unwrap();
  let max_col = *map.keys().map(|(_r,c)| c).max().unwrap();

  for row in 0..=max_row {
    for col in 0..=max_col {
      if let Some(p) = map.get(&(row,col)) {
        let any_units = units.iter().filter(|u| u.position == (row,col) && u.hit_points > 0).collect::<Vec<&Unit>>();
        if history.contains(&(row,col)) { print!("O"); }
        else if any_units.len() > 0 {
          match any_units[0].kind {
            UnitType::Goblin => { print!("G"); }
            UnitType::Elf => { print!("E"); }
            UnitType::None => { panic!("None in the units"); }
          }
        } else {
          print!("{}", p);
        }
      }
    }
    print!("  {}", units.iter().filter(|u| u.position.0 == row).map(|u| format!("{:?}({})", u.kind, u.hit_points)).collect::<String>());
    println!();
  }
}

pub fn print_map(map : &HashMap<(isize,isize),char>, units : &Vec<Unit>) {
  let max_row = *map.keys().map(|(r,_c)| r).max().unwrap();
  let max_col = *map.keys().map(|(_r,c)| c).max().unwrap();
  for row in 0..=max_row {
    for col in 0..=max_col {
      if let Some(p) = map.get(&(row,col)) {
        let any_units = units.iter().filter(|u| u.position == (row,col) && u.hit_points > 0).collect::<Vec<&Unit>>();
        if any_units.len() > 0 {
          match any_units[0].kind {
            UnitType::Goblin => { print!("G"); }
            UnitType::Elf => { print!("E"); }
            UnitType::None => { panic!("None in the units"); }
          }
        } else {
          print!("{}", p);
        }
      }
    }
    print!("  {}", units.iter().filter(|u| u.position.0 == row && u.hit_points > 0).map(|u| format!("{:?}({})", u.kind, u.hit_points)).collect::<String>());
    println!();
  }
}
pub fn move_unit(map : &HashMap<(isize,isize),char>, units : &mut Vec<Unit>, u : &Unit) {
  // let max_row : usize= *map.keys().map(|(r,_c)| r).max().unwrap() as usize;
  // let max_col : usize = *map.keys().map(|(_r,c)| c).max().unwrap() as usize;

  // let mut cache = HashMap::new();

  let mut candidates = vec![];
  let mut clone_units = units.clone();
  clone_units.sort_by_key(|my_unit| manhattan_distance(my_unit.position, u.position));

  for target in clone_units.iter().filter(|my_unit| my_unit.hit_points > 0 && my_unit.kind != u.kind) {
    // println!("Checking out {:?}", target);
    for my_n in get_neighbors(map, units, &u.position) {
      for n in get_neighbors(map, units, &target.position) {
        if candidates.len() > 0 && manhattan_distance(n, my_n) > candidates.iter().map(|(d,_p1,_p2)| *d).min().unwrap() { 
          // println!("for sure too far"); 
        // } else if let Some(d) = path(map, n, my_n, &vec![], &clone_units, 0, 2*max_row+max_col, &mut cache) {
        } else if let Some(d) = other_path(map, n, my_n, units) {
          // println!("found path {:?} to {:?}", u.position, n);
          candidates.push((d.1 as isize,n,my_n));
        }
      }
    }
  }

  if candidates.len() == 0 { return; }
  // println!("{:?} has candidates {}", u, candidates.len());
  candidates.sort_by_key(|k| k.0*10000000 + k.1.0*100 + k.1.1);
  
  let top_candidate = candidates[0];
  let target_spot = top_candidate.2; 
  
  // println!("MOVE {:?} to {:?}", u.position, target_spot);
  let target_position = units.iter().position(|my_unit| my_unit.position==u.position && my_unit.hit_points > 0).unwrap();
  let mut target = units.get_mut(target_position).unwrap();
  target.position = target_spot;

}

pub fn attack_unit(map : &HashMap<(isize,isize),char>, units : &mut Vec<Unit>, u : &Unit, elf_power : i32) -> bool {
  let mut candidates = vec![];
  for n in get_neighbors_with_units(map, &u.position) {
    if let Some(n_o) = units.iter().filter(|my_unit| my_unit.position == n && my_unit.hit_points > 0).nth(0) {
      if n_o.kind != u.kind {
        candidates.push(n_o);
      }
    }
  }
  if candidates.len() == 0 { return false; }

  candidates.sort_by_key(|k| k.hit_points * 10000000 + k.position.0 as i32 *100 + k.position.1 as i32);

  let target_spot = candidates[0].position;
  // println!("ATTACK {:?} v {:?}", u.position, target_spot);
  let target_position = units.iter().position(|u| u.position==target_spot && u.hit_points > 0).unwrap();
  let mut target = units.get_mut(target_position).unwrap();
  if u.kind == UnitType::Elf {
    target.hit_points = target.hit_points - elf_power;
  } else {
    target.hit_points = target.hit_points - 3;
  }
  return true;
}

pub fn load_grid(lines : &Vec<String>) -> (HashMap<(isize,isize),char>, Vec<Unit>) {
  let mut units : Vec<Unit> = vec![];

  let mut map = common::make_map(&lines);
  // let spot = common::get_spot_on_map(&map, 1, 1, '.');
  // assert!(spot == 'G');
  
  let max_row = *map.keys().map(|(r,_c)| r).max().unwrap();
  let max_col = *map.keys().map(|(_r,c)| c).max().unwrap();
  for row in 0..=max_row {
    for col in 0..=max_col {
      if let Some(p) = map.get_mut(&(row,col)) {
        if *p == 'G' {
          units.push(Unit{ kind : UnitType::Goblin, position : (row,col), hit_points  : 200 });
          *p = '.';
        } else if *p == 'E' {
          units.push(Unit{ kind : UnitType::Elf, position : (row,col), hit_points  : 200 });
          *p = '.';
        }
      }

    }
  }
  return (map,units);
}

fn dead_elves(units : &Vec<Unit>) -> usize {
  return units.iter().filter(|u| u.kind == UnitType::Elf && u.hit_points <= 0).count();
}

pub fn run_simulation(lines : &Vec<String>, candidate_power : i32) -> (UnitType, i32) {
  let (map, mut units) = load_grid(lines);
  // print_map(&map, &units);
  let mut rounds = 0;

  loop { // rounds
    units.sort_by_key(|k| k.position.0*1000+k.position.1);
    for i in 0..units.len() {
      if units[i].hit_points > 0 {
        let u = units[i].clone();
        // println!("Working unit {:?}", u);
        if !attack_unit(&map, &mut units, &u, candidate_power) {
          move_unit(&map,&mut units,&u.clone());
          let u = units[i].clone();
          attack_unit(&map,&mut units,&u.clone(),candidate_power);
        }

        if dead_elves(&units) > 0 { 
          // println!("One dead elf after {} rounds at power {}", rounds, candidate_power); 

          return (UnitType::Goblin,rounds * units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points));
        }
        if units.iter().filter(|u| u.kind == UnitType::Goblin && u.hit_points > 0).count() == 0
        {
          if units[i+1..].iter().filter(|my_unit| my_unit.hit_points > 0).count() == 0 { rounds = rounds + 1; }

          // println!("Elves win {} {} {}",units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points),  
          //     rounds, rounds * units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points));
          // print_map(&map, &units);
          return (UnitType::Elf,rounds * units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points));
        }  
      }
    }

    rounds = rounds + 1;
    // println!("After {} round winner {:?}", rounds, winner);
    // print_map(&map, &units);
  }
}

pub fn solve_part2(file_name : String) -> i32 {
  let lines = common::read_input(file_name);

  let mut lo_power = 4;
  let mut hi_power = 50;
  let mut mid_power;
  let mut result;

  loop {
    mid_power = (lo_power + hi_power)/2;
    println!("{} {} {}", lo_power, mid_power, hi_power);
    result = run_simulation(&lines, mid_power);
    let winner = result.0;

    if winner == UnitType::Goblin {
      lo_power = mid_power;
    } else {
      hi_power = mid_power;
    }
    if hi_power-lo_power <= 1 { break; }
  }

  if mid_power != hi_power {
    result = run_simulation(&lines, hi_power);
  }

  println!("Day 15 part 2 candidate power {} outcome {}!", hi_power, result.1);
  return result.1;
}


pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let (map, mut units) = load_grid(&lines);
  let mut rounds = 0;
  print_map(&map, &units);

  loop {
    units.sort_by_key(|k| k.position.0*1000+k.position.1);
    for i in 0..units.len() {
      if units[i].hit_points > 0 {
        let u = units[i].clone();
        // println!("Working unit {:?}", u);
        if !attack_unit(&map, &mut units, &u, 3) {
          move_unit(&map,&mut units,&u.clone());
          let u = units[i].clone();
          attack_unit(&map,&mut units,&u.clone(),3);
        }

        if units.iter().filter(|u| u.kind == UnitType::Goblin && u.hit_points > 0).count() == 0 || 
            units.iter().filter(|u| u.kind == UnitType::Elf && u.hit_points > 0).count() == 0 
        {
          if i == units.len()-1 { rounds = rounds + 1; }
          print_map(&map, &units);
          println!("Day 15 part 1 round:{} hit_points: {} outcome:{}",units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points),  
              rounds, rounds * units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points));
          return (rounds * units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points)).try_into().unwrap();
        }  
      }
    }

    rounds = rounds + 1;
    if rounds % 1 == 0 {
      println!("After {} round", rounds);
      print_map(&map, &units);
    }

    // 250648 solved
  }

}
