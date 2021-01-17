#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle15_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(36334==super::solve("./inputs/puzzle15-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle15_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle15.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use std::collections::HashMap;
use cached::proc_macro::cached;
use cached::SizedCache;

#[derive(Debug,Clone,PartialEq)]
pub enum UnitType {
  Goblin,
  Elf,
}

#[derive(Debug,Clone)]
pub struct Unit {
  kind : UnitType,
  position : (isize,isize),
  hit_points : i32,
}

fn get_neighbors(map : &HashMap<(isize,isize),char>, p : &(isize,isize)) -> Vec<(isize,isize)> {
  let v = vec![(-1,0), (0,-1), (0,1), (1,0)];

  v.iter().map(|n| (p.0+n.0,p.1+n.1)).filter(|n| map.get(n) != None && *map.get(n).unwrap() == '.').collect::<Vec<(isize,isize)>>().to_vec()
}

fn manhattan_distance (p1 : (isize,isize), p2 : (isize,isize)) -> isize {
  return (p1.0-p2.0).abs()+(p1.1-p2.1).abs();
}
// #[cached(
//   type = "SizedCache<String, Option<usize>>",
//   create = "{ SizedCache::with_size(10000) }",
//   convert = r#"{ format!("{},{},{},{},{:?},{:?}",point_a.0,point_a.1,point_b.0,point_b.1,history,units) }"#
// )]
fn path(map : &HashMap<(isize,isize),char>, 
        point_a : (isize,isize), 
        point_b : (isize,isize), 
        history : &Vec<(isize,isize)>, 
        units : &Vec<Unit>, max : usize) -> Option<usize> {
  
  let mut min_distance = None;
  // if last_min == None {
  //   min_distance = 9999;
  // } else {
  //   min_distance = last_min.unwrap();
  // }
  if point_a == point_b { 
    // println!("Found! {}", history.len()); 
    return Some(history.len()); 
  }
  if history.len() >= max { return None; }

  let mut neighbors = get_neighbors(map, &point_a);
  neighbors.sort_by_key(|k| manhattan_distance(*k, point_b));

  for n in neighbors {
    // println!("History {:?}, {:?} {:?}", point_a, point_b, history.len());
    // print_map_history(map, units, &history);
    if n == point_b {
      if min_distance == None || history.len()+1 < min_distance.unwrap() {
        min_distance = Some(history.len()+1);  
        // println!("Found! {}", min_distance.unwrap()); 
        break; }
    } 
    // if let Some(n_o) = map.get(&n) {
    if min_distance == None || history.len()+1 < min_distance.unwrap() {
      if units.iter().filter(|u| u.position == n && u.hit_points > 0).count() == 0  {
        if !history.contains(&n) {
          let mut new_history = history.clone();
          new_history.push(n);
          if let Some(d) = path(&map, n, point_b, &new_history, &units, max) {
            if min_distance == None || d < min_distance.unwrap() {min_distance = Some(d); }
          }
        }
      }
    }
  }
  return min_distance;
  // if min_distance < 9999 { return Some(min_distance) };

  // return None;
}

pub fn print_map_history(map : &HashMap<(isize,isize),char>, units : &Vec<Unit>, history : &Vec<(isize,isize)>) {
  let max_row = *map.keys().map(|(r,c)| r).max().unwrap();
  let max_col = *map.keys().map(|
    (r,c)| c).max().unwrap();
  // print!("{}[2J", 27 as char);
  for row in 0..=max_row {
    for col in 0..=max_col {
      if let Some(p) = map.get(&(row,col)) {
        let any_units = units.iter().filter(|u| u.position == (row,col) && u.hit_points > 0).collect::<Vec<&Unit>>();
        if history.contains(&(row,col)) { print!("O"); }
        else if any_units.len() > 0 {
          match any_units[0].kind {
            UnitType::Goblin => { print!("G"); }
            UnitType::Elf => { print!("E"); }
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
  let max_row = *map.keys().map(|(r,c)| r).max().unwrap();
  let max_col = *map.keys().map(|(r,c)| c).max().unwrap();
  for row in 0..=max_row {
    for col in 0..=max_col {
      if let Some(p) = map.get(&(row,col)) {
        let any_units = units.iter().filter(|u| u.position == (row,col) && u.hit_points > 0).collect::<Vec<&Unit>>();
        if any_units.len() > 0 {
          match any_units[0].kind {
            UnitType::Goblin => { print!("G"); }
            UnitType::Elf => { print!("E"); }
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
pub fn move_unit(map : &HashMap<(isize,isize),char>, units : &mut Vec<Unit>, u : &Unit) {
  let max_row : usize= *map.keys().map(|(r,c)| r).max().unwrap() as usize;
  let max_col : usize = *map.keys().map(|(r,c)| c).max().unwrap() as usize;

  let mut candidates = vec![];
  let clone_units = units.clone();
  for target in clone_units.iter().filter(|my_unit| my_unit.hit_points > 0 && my_unit.position != u.position && my_unit.kind != u.kind) {
    println!("Checking out {:?}", target);
    // for n in get_neighbors(map, &target.position) {
      // println!("Calling path on {:?} {:?}", target.position, u.position);
      let n = target.position;
      if let Some(d) = path(map, n, u.position, &vec![], units, max_row+max_col) {
        // if units.iter().filter(|u| u.position == n && u.hit_points > 0).count() == 0 {
          println!("found path {:?} to {:?}", u.position, n);
          candidates.push((d as isize,n));
        // }
      }
    // }
  }
  
  // println!("{:?}", candidates);
  if candidates.len() == 0 { return; }

  candidates.sort_by_key(|k| k.0*10000000 + k.1.0*100 + k.1.1);
  
  let top_candidate = candidates[0];
  let mut target_spot = u.position;
  for n in get_neighbors(map, &u.position) {
    if units.iter().filter(|u| u.position == n && u.hit_points > 0).count() == 0 {
      if let Some(d) = path(map, n, top_candidate.1, &vec![], units, max_row+max_col) {
        if top_candidate.0 > d.try_into().unwrap() {
          target_spot = n;
          break;
        }
      }
    }
  }
  // println!("MOVE {:?} to {:?}", u.position, target_spot);
  let target_position = units.iter().position(|my_unit| my_unit.position==u.position && my_unit.hit_points > 0).unwrap();
  let mut target = units.get_mut(target_position).unwrap();
  target.position = target_spot;

}

pub fn attack_unit(map : &HashMap<(isize,isize),char>, units : &mut Vec<Unit>, u : &Unit) -> bool {
  let mut candidates = vec![];
  for n in get_neighbors(map, &u.position) {
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
  target.hit_points = target.hit_points - 3;

  return true;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");

  let mut units : Vec<Unit> = vec![];

  let mut map = common::make_map(&lines);
  // let spot = common::get_spot_on_map(&map, 1, 1, '.');
  // assert!(spot == 'G');
  
  let max_row = *map.keys().map(|(r,c)| r).max().unwrap();
  let max_col = *map.keys().map(|(r,c)| c).max().unwrap();
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
  let mut rounds = 0;
  print_map(&map, &units);

  // let u = units[0].clone();
  // if !attack_unit(&map, &mut units, &u) {
  //   move_unit(&map, &mut units,&u);
  //   attack_unit(&map, &mut units, &u);
  // }
  loop {
    units.sort_by_key(|k| k.position.0*1000+k.position.1);
    for i in 0..units.len() {
      if units[i].hit_points > 0 {
        let u = units[i].clone();
        // println!("Working unit {:?}", u);
        if !attack_unit(&map, &mut units, &u) {
          move_unit(&map,&mut units,&u.clone());
          let u = units[i].clone();
          attack_unit(&map,&mut units,&u.clone());
        }

        if units.iter().filter(|u| u.kind == UnitType::Goblin && u.hit_points > 0).count() == 0 {
          println!("{} {} {}",units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points),  
              rounds, rounds * units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points));
          print_map(&map, &units);
          return (rounds * units.iter().filter(|u| u.hit_points > 0).fold(0, |acc,u| acc+u.hit_points)).try_into().unwrap();
        }  
      }
    }

    rounds = rounds + 1;
    if rounds % 1 == 0 {
      println!("After {} round", rounds);
      print_map(&map, &units);
    }
    // if rounds == 47 { break; }
  }

  // return 0;
}
