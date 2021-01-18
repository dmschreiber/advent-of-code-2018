#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle15_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(18740==super::solve("./inputs/puzzle15-test.txt".to_string()));
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

fn path(map : &HashMap<(isize,isize),char>, 
        point_a : (isize,isize), 
        point_b : (isize,isize), 
        history : &Vec<(isize,isize)>, 
        units : &Vec<Unit>, so_far : usize, max : usize,
        cache : &mut HashMap<String,Option<usize>>) -> Option<usize> {
  
  let mut min_distance = None;
  if point_a == point_b { 
    println!("Found! returning zero {:?}", history); 
    return Some(0); 
  }

  if so_far >= max { return None; }
  let key = format!("{},{},{},{}", point_a.0, point_a.1, point_b.0, point_b.1);
  let mut distance;
  // if let Some(d) = cache.get(&key) {
  //   distance = *d;
  //   // println!("Got {:?} from key {}", distance, key);
  //   return distance;
  // }

  let mut neighbors = get_neighbors(map, units, &point_a);
  // neighbors.sort_by_key(|k| manhattan_distance(*k, point_b));

  for n in neighbors {
    // println!("History {:?}, {:?}", point_a, so_far);
    // print_map_history(map, units, &history);
    if n == point_b {
      if min_distance == None ||so_far+1 < min_distance.unwrap() {

        min_distance = Some(1);  
        println!("Found! min_distance {:?}->{:?} {:?}", point_a, point_b, history); 
        print_map_history(map, units, history);
        break; 
      }
    } else if min_distance == None || so_far+1 <= min_distance.unwrap() {
        if !history.contains(&n) {
          let mut new_history = history.clone();
          new_history.push(n);
          distance = path(&map, n, point_b, &new_history, &units, so_far+1, max, cache);

          if let Some(d) = distance {
            if min_distance == None || d < min_distance.unwrap() {
              min_distance = Some(d+1); 
              if d as isize == manhattan_distance(n,point_b) { break; }
            }
          }
        }
      }
    }
  
  // let key = format!("{},{},{},{}", point_a.0, point_a.1, point_b.0, point_b.1);
  // // println!("Put {:?} to key {}", min_distance, key);
  // if let Some(c) = cache.get_mut(&key) {
  //   if *c == None || min_distance < *c {
  //     *c = min_distance;
  //   }
  // } else {
  //   cache.insert(key,min_distance);
  // }
  return min_distance;
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
  let max_row : usize= *map.keys().map(|(r,_c)| r).max().unwrap() as usize;
  let max_col : usize = *map.keys().map(|(_r,c)| c).max().unwrap() as usize;

  let mut cache = HashMap::new();

  let mut candidates = vec![];
  let clone_units = units.clone();
  for target in clone_units.iter().filter(|my_unit| my_unit.hit_points > 0 && my_unit.position != u.position && my_unit.kind != u.kind) {
    // println!("Checking out {:?}", target);
    for my_n in get_neighbors(map, units, &u.position) {

      for n in get_neighbors(map, units, &target.position) {
        // println!("Calling path on {:?} {:?}", target.position, u.position);
        if let Some(d) = path(map, my_n, n, &vec![], &clone_units, 0, 4*max_row+max_col, &mut cache) {
            // println!("found path {:?} to {:?}", u.position, n);
            candidates.push((d as isize,n,my_n));
        }
      }
    }
  }

  // println!("{:?}", candidates);
  if candidates.len() == 0 { return; }

  candidates.sort_by_key(|k| k.0*10000000 + k.1.0*100 + k.1.1);
  
  let top_candidate = candidates[0];
  let mut target_spot = u.position;
  for my_n in get_neighbors(map, units, &u.position) {
      if let Some(d) = path(map, my_n, top_candidate.1, &vec![], &clone_units, 0, 4*max_row+max_col, &mut cache) {
        if top_candidate.0 == d.try_into().unwrap() {
          println!("{:?} moves to {:?}", u, top_candidate);
          let mut cache = HashMap::new();

         if path(map, top_candidate.1, my_n, &vec![], units, 0, max_row+max_col, &mut cache) !=
            path(map, my_n, top_candidate.1, &vec![], units, 0, max_row+max_col, &mut cache) {
            println!("Compare {:?} to {:?} from {:?} to {:?}", 
              path(map, my_n, top_candidate.1, &vec![], units, 0, max_row+max_col, &mut cache), 
              path(map, top_candidate.1, my_n, &vec![], units, 0, max_row+max_col, &mut cache),
              my_n,
              top_candidate.1);
            println!("{:?}", get_neighbors(map, units, &u.position));
            panic!("reverse direction different length ({:?} to {:?})", my_n, top_candidate.1);
          }
          target_spot = my_n;
          break;
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

        if units.iter().filter(|u| u.kind == UnitType::Goblin && u.hit_points > 0).count() == 0 || 
            units.iter().filter(|u| u.kind == UnitType::Elf && u.hit_points > 0).count() == 0 
        {
          if i == units.len()-1 { rounds = rounds + 1; }
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

    // 254784 too high
    // ?? 2737 85 232645 -- too low
    // ?? 2618 96 251328 -- too high
  }

  // return 0;
}