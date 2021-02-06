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
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle24.txt".to_string());
  }
}

use crate::common;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
  pub static ref GROUP_REGEX: Regex = Regex::new(r"^([0-9+]+) units each with ([0-9+]+) hit points \((.+)\) with an attack that does ([0-9+]+) ([a-z]+) damage at initiative ([0-9+]+)$").unwrap();
  pub static ref SPECIAL_GROUP_REGEX: Regex = Regex::new(r"^([0-9+]+) units each with ([0-9+]+) hit points with an attack that does ([0-9+]+) ([a-z]+) damage at initiative ([0-9+]+)$").unwrap();
}

#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub enum Army {
  ImmuneSystem,
  Infection,
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

    let attack_type = inner[3].to_string();
    let attack_strength = inner[3].parse::<u32>().unwrap();
    let initiative = inner[5].parse::<u32>().unwrap();
    return Group{ index : index, army: army.clone(), units : units, hit_points : hp, immune : immune, weak : weak, attack_type : attack_type, attack_strength : attack_strength, initiative : initiative };

  }
  panic!("invalid format {}", expression);
}

pub fn fight(groups : &Vec<Group>, targeting : &HashMap<(Army,u32),Option<(Army,u32)>>) -> Vec<Group> {
  let mut new_groups : Vec<Group> = vec![];
  let mut killed_groups : Vec<(Army,u32)> = vec![];

  for g in groups {
    let target = targeting.get(&(g.army,g.index)).unwrap();
    if let Some((target_army,target_index)) = target {
      let target_group = &groups.iter().filter(|which| which.army == *target_army && which.index == *target_index).map(|which| which.clone()).collect::<Vec<Group>>()[0];

      if killed_groups.iter().filter(|which| **which == (g.army,g.index) ).count() == 0 {
        let damage;
        if new_groups.iter().filter(|which| which.army == g.army && which.index == g.index).count() > 0 {
          damage = new_groups.iter().filter(|which| which.army == g.army && which.index == g.index).map(|which| which.attack_power(target_group)).sum();
        } else {
          damage = g.attack_power(target_group);
        }

        // println!("Damage {}, target units {}, target hit points {}", damage, target_group.units, target_group.hit_points);
        if damage < (target_group.units * target_group.hit_points) {
          println!("{:?} group {} attacks defending group {}, killing {} units", g.army, g.index, target_group.index, damage / target_group.hit_points);
          let remaining_units = target_group.units - ( damage / target_group.hit_points );
          let mut new_group = target_group.clone();
          new_group.units = remaining_units;
          new_groups.push(new_group);
        } else {
          println!("{:?} group {} attacks defending group {}, killing the group ({} units)", g.army, g.index, target_group.index, damage / target_group.hit_points);
          // killed_groups.push((target_group.army,target_group.index));
        }
      } else { // the group that targeted me was killed first
        println!("The group that targeted {:?} group {} me was killed first", target_group.army, target_group.index);
        new_groups.push(target_group.clone());

      }
    }

    // if the group was not targeted
    if targeting.values().filter(|target| **target == Some((g.army,g.index)) ).count() == 0 {
      new_groups.push(g.clone());
    }
  }
  
  return new_groups;
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


  loop {
    println!("Current standing");
    for g in &groups {
      println!("{:?} group {} contains {} units ({} effective power)", g.army, g.index, g.units, g.effective_power());
    }

    println!("Tageting");
    let mut targeting : HashMap<(Army,u32),Option<(Army,u32)>> = HashMap::new();
    // targeting in decreasing order of effective power; tie breaker highest initiative
    groups.sort_by_key(|k| (k.effective_power() as i64 * 100 + k.initiative as i64) * -1);
    for g in &groups {
      let mut max_damage = 0;
      let mut max_effective_power = 0;
      let mut max_initiative = 0;

      let mut target = None;
      for other_g in &groups {
        if g.army != other_g.army && targeting.values().filter(|which| **which == Some((other_g.army,other_g.index)) ).count() == 0 {
          // println!("{:?} aattacks {:?}", g, other_g);
          if g.attack_power(&other_g) > max_damage {
            max_damage = g.attack_power(&other_g);
            target = Some((other_g.army,other_g.index));
            max_effective_power = other_g.effective_power();
            max_initiative = other_g.initiative;
          } else if g.attack_power(&other_g) == max_damage {
            if other_g.effective_power() > max_effective_power {
              max_effective_power = other_g.effective_power();
              target = Some((other_g.army,other_g.index));
              max_initiative = other_g.initiative;
            } else if other_g.effective_power() == max_effective_power {
              if other_g.initiative > max_initiative {
                max_initiative = other_g.initiative;
                target = Some((other_g.army,other_g.index));
              }
            }
          }
          println!("{:?} group {} would deal defending group {} {} damage", g.army, g.index, other_g.index, g.attack_power(&other_g));
        }
      }
      targeting.insert((g.army,g.index),target);
    }

    println!("Fighting");
    // println!("{:?}", targeting);

    // println!();
    groups.sort_by_key(|k| k.initiative as i64 * -1);
    groups = fight(&groups,&targeting);

    if groups.iter().filter(|g| g.army == Army::Infection).map(|which| which.units as i64).sum::<i64>() == 0 {
      println!("Immune System Won, Returning {}", groups.iter().filter(|g| g.army == Army::ImmuneSystem).map(|which| which.units as i64).sum::<i64>());
      return groups.iter().filter(|g| g.army == Army::ImmuneSystem).map(|which| which.units as i64).sum::<i64>();
    }
    else if groups.iter().filter(|g| g.army == Army::ImmuneSystem).map(|which| which.units as i64).sum::<i64>() == 0 {
      println!("Infection Won, Returning {}", groups.iter().filter(|g| g.army == Army::Infection).map(|which| which.units as i64).sum::<i64>());
      return groups.iter().filter(|g| g.army == Army::Infection).map(|which| which.units as i64).sum::<i64>();
    }
  }

  // 25872 too low
}
