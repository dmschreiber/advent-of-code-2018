#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle24_test() {
    assert!(common::format_binary(10)=="1010");
    let g = super::extract_group(&"4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4".to_string(), &super::Army::ImmuneSystem, 1);
    assert!(g==super::Group{ index: 1, 
                      army : super::Army::ImmuneSystem,
                      units : 4485, 
                      hit_points: 2961,
                      immune : vec!["radiation".to_string()],
                      weak : vec!["fire".to_string(), "cold".to_string()],
                      attack_strength : 12,
                      attack_type : "slashing".to_string(),
                      initiative: 4});
    assert!(5216==super::solve("./inputs/puzzle24-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle24_prod() {
    assert!(26937==super::solve("./inputs/puzzle24.txt".to_string()));
  }
}

use crate::common;
use std::collections::HashMap;
use regex::Regex;
use std::cmp::Reverse;

lazy_static! {
  pub static ref GROUP_REGEX: Regex = Regex::new(r"^([0-9+]+) units each with ([0-9+]+) hit points \((.+)\) with an attack that does ([0-9+]+) ([a-z]+) damage at initiative ([0-9+]+)$").unwrap();
  pub static ref SPECIAL_GROUP_REGEX: Regex = Regex::new(r"^([0-9+]+) units each with ([0-9+]+) hit points with an attack that does ([0-9+]+) ([a-z]+) damage at initiative ([0-9+]+)$").unwrap();
}

#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub enum Army {
  ImmuneSystem,
  Infection,
  None,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Group {
  army : Army,
  index : u32,
  units : u32,
  hit_points : u32,
  immune : Vec<String>,
  weak : Vec<String>,
  attack_type : String,
  attack_strength : u32,
  initiative : u32,
}
impl Group {
  fn effective_power (self : &Self) -> u32 {
    return self.units * self.attack_strength;
  }

  fn attack_power (self : &Self, target : &Group) -> u32 {
    // println!("{} attacks weak {:?}", self.attack_type, target.weak );
    if target.immune.contains(&self.attack_type) { return 0; }
    else if target.weak.contains(&self.attack_type) { return self.effective_power()*2; }
    else { return self.effective_power(); }
  }
}

// 4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
fn extract_group(expression : &String, army : &Army, index : u32) -> Group {
  if let Some(inner) = GROUP_REGEX.captures(expression) {
    let units = inner[1].parse::<u32>().unwrap();
    let hp = inner[2].parse::<u32>().unwrap();
    let weak_immune = inner[3].split("; ");

    let mut immune = vec![];
    let mut weak = vec![];
    for item in weak_immune {
      if &item[0..6] == "immune" {
        immune = item[10..].split(", ").collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect::<Vec<String>>();
      } else if &item[0..4] == "weak" {
        weak = item[8..].split(", ").collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect::<Vec<String>>();
      }
    }

    let attack_type = inner[5].to_string();
    let attack_strength = inner[4].parse::<u32>().unwrap();
    let initiative = inner[6].parse::<u32>().unwrap();
    return Group{ index : index, army: army.clone(), units : units, hit_points : hp, immune : immune, weak : weak, attack_type : attack_type, attack_strength : attack_strength, initiative : initiative };
  } else if let Some(inner) = SPECIAL_GROUP_REGEX.captures(expression) {
    let units = inner[1].parse::<u32>().unwrap();
    let hp = inner[2].parse::<u32>().unwrap();

    let immune = vec![];
    let weak = vec![];

    let attack_type = inner[4].to_string();
    let attack_strength = inner[3].parse::<u32>().unwrap();
    let initiative = inner[5].parse::<u32>().unwrap();
    return Group{ index : index, army: army.clone(), units : units, hit_points : hp, immune : immune, weak : weak, attack_type : attack_type, attack_strength : attack_strength, initiative : initiative };

  }
  panic!("invalid format {}", expression);
}

pub fn fight(groups : &mut Vec<Group>, targeting : &HashMap<(Army,u32),Option<(Army,u32)>>) -> bool {
  // let mut new_groups : Vec<Group> = vec![];
  // let mut killed_groups : Vec<(Army,u32)> = vec![];
  let group_count = groups.len();
  let mut damage_done = 0;

  for i in 0..group_count {
    let (g_army, g_index) = (groups.get(i).unwrap().army,groups.get(i).unwrap().index);
    if let Some(target) = targeting.get(&(g_army,g_index)) {
      if let Some((target_army,target_index)) = target {
        let target_pos = groups.iter().position(|which|  which.army == *target_army && which.index == *target_index).unwrap();
        // let target_group = &groups.iter().filter(|which| which.army == *target_army && which.index == *target_index).map(|which| which.clone()).collect::<Vec<Group>>()[0];
        let g = groups.get(i).unwrap().clone();
        let mut target_group = groups.get_mut(target_pos).unwrap();

        if g.units == 0 { continue; }
        if g.units > 0 {
          let damage = g.attack_power(&target_group);

          if damage < target_group.units * target_group.hit_points {
            damage_done = damage_done + damage / target_group.hit_points ;            
            target_group.units = target_group.units - ( damage / target_group.hit_points );
            // println!("{:?} group {} attacks defending group {}, killing {} units", g.army, g.index, target_group.index, damage / target_group.hit_points);
          } else {
            // println!("{:?} group {} attacks defending group {}, killing {} units", g.army, g.index, target_group.index, target_group.units);
            damage_done = damage_done + target_group.units;
            target_group.units = 0;
          }
        }
      }
    }
  } 
  return damage_done > 0;
  
}

#[allow(dead_code)]
fn display_standing(groups : &Vec<Group>) {
  println!("Current standing");
  for g in groups {
    if g.units > 0 {
      println!("{:?} group {} contains {} units ({} effective power)", g.army, g.index, g.units, g.effective_power());
    }
  }
}

pub fn simulate(starting_groups : &Vec<Group>, boost : u32) -> (Army, u32) {
  let mut groups = starting_groups.clone();
  for g in groups.iter_mut() {
    if g.army == Army::ImmuneSystem { g.attack_strength += boost; }
  }

  loop {
    // println!("Tageting");
    let mut targeting : HashMap<(Army,u32),Option<(Army,u32)>> = HashMap::new();

    groups.sort_by_key(|k| Reverse((k.effective_power(), k.initiative)) );
    for g in &groups {
      let mut max_damage = 0;

      let mut target = None;
      if g.units > 0 {
        for other_g in &groups {
          if other_g.units > 0 && g.army != other_g.army && targeting.values().filter(|which| **which == Some((other_g.army,other_g.index)) ).count() == 0 {

            if g.attack_power(&other_g) > max_damage {
              max_damage = g.attack_power(&other_g);
              target = Some((other_g.army,other_g.index));
            }
            // println!("{:?} group {} would deal defending group {} {} damage", g.army, g.index, other_g.index, g.attack_power(&other_g));
          }          
        }

        targeting.insert((g.army,g.index),target);
      }
    }

    groups.sort_by_key(|k| Reverse(k.initiative ) );
    if !fight(&mut groups,&targeting) {
      // println!("Tie");
      return (Army::None,0);
    }

    if groups.iter().filter(|g| g.army == Army::Infection && g.units > 0).count() == 0 {
      return (Army::ImmuneSystem,groups.iter().filter(|g| g.army == Army::ImmuneSystem).map(|which| which.units).sum::<u32>());
    }
    else if groups.iter().filter(|g| g.army == Army::ImmuneSystem && g.units > 0).count() == 0 {
      return (Army::Infection,groups.iter().filter(|g| g.army == Army::Infection).map(|which| which.units).sum::<u32>());
    }
  }
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let mut groups = vec![];
  let mut group_index = 1;
  let mut current_type = Army::ImmuneSystem;
  for l in lines {
    if l == "Immune System:" {
      current_type = Army::ImmuneSystem;

      // do nothing
    } else if l == "Infection:" {
      current_type = Army::Infection;
      group_index = 1;
    } else if l.len() > 0 {
      groups.push(extract_group(&l, &current_type, group_index));
      group_index = group_index + 1;
    }
  }
  
  // Part 1
  let (_winner,part1_units) = simulate(&groups, 0);
  println!("Day 24 part 1 {:?} wins with {} units", _winner, part1_units);

  // Part 2
  let mut hi_boost = 1000000;
  let mut lo_boost = 0;

  while hi_boost - lo_boost > 1 {
    let mid_boost = (hi_boost+lo_boost)/2;

    let (winner,_units) = simulate(&groups, mid_boost);

    if winner == Army::ImmuneSystem {
      hi_boost = mid_boost;
    } else {
      lo_boost = mid_boost;
    }
  }
  println!("Final boost - {}", hi_boost);
  let (winner,units) = simulate(&groups, hi_boost);
  println!("Winner {:?} with {} units", winner, units);

  return part1_units as i64;
}
