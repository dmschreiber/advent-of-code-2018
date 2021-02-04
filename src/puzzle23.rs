#[cfg(test)]
mod tests {
  #[test]
  pub fn puzzle23_test() {
    assert!(((41289914,12552653,-7638886),70344373)==super::parse_pos(&"pos=<41289914,12552653,-7638886>, r=70344373".to_string()));
    assert!(36==super::solve("./inputs/puzzle23-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle23_prod() {
    super::solve("./inputs/puzzle23.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;
use std::collections::HashMap;

// pos=<41289914,12552653,-7638886>, r=70344373
lazy_static! {
  pub static ref POS_REGEX: Regex = Regex::new(r"^pos=<(-?[0-9+]+),(-?[0-9+]+),(-?[0-9+]+)>, r=([0-9+]+)$").unwrap();
}

fn parse_pos(expression : &String) -> ((isize,isize,isize),usize) {
  
  if let Some(args) = POS_REGEX.captures(expression) {
    let point = (args[1].parse::<isize>().unwrap(),args[2].parse::<isize>().unwrap(),args[3].parse::<isize>().unwrap());
    let radius = args[4].parse::<usize>().unwrap();
    return (point, radius);
  }

  panic!("bad format");
}

fn manhattan_distance(p1 : (isize,isize,isize), p2 : (isize,isize,isize)) -> usize {
  return ( (p1.0-p2.0).abs() + (p1.1-p2.1).abs() + (p1.2-p2.2).abs() ) as usize;
}

fn get_field(spot : (isize,isize,isize), bots : &Vec<((isize,isize,isize),usize)>, map : &mut HashMap<(isize,isize,isize),f64>) -> f64 {
  if let Some(field) = map.get(&spot) {
    return *field;
  } else {
    let experiment : f64 = bots.iter().map(|(a,r)| { if manhattan_distance(*a, spot) > *r { *r as f64 / manhattan_distance(*a, spot) as f64 } else { 1f64 } } ).sum();
    map.insert(spot,experiment);
    return experiment;
  }

}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let nanobots = lines.iter().map(|l| parse_pos(l)).collect::<Vec<((isize,isize,isize),usize)>>();

  let biggest = nanobots.iter().map(|(_a,r)| *r).max().unwrap();
  let pos = nanobots.iter().filter(|(_a,r)| *r==biggest).map(|(a,_r)| *a).collect::<Vec<(isize,isize,isize)>>()[0];

  let in_range = nanobots.iter().filter(|(a,_r)| manhattan_distance(*a, pos) <= biggest).count();
  println!("{:?} - in range {}", pos, in_range);

// found (52176047, 17543225, 46828677) - 116547949

  let positions = nanobots.iter().map(|(a,_r)| *a).collect::<Vec<(isize,isize,isize)>>(); 
  let sum_p : (isize,isize,isize) = positions.iter().fold((0,0,0),|acc,p| (acc.0+p.0,acc.1+p.1,acc.2+p.2) );
  let avg_p = (sum_p.0.checked_div(positions.len().try_into().unwrap()).unwrap(), 
        sum_p.1.checked_div(positions.len().try_into().unwrap()).unwrap(), 
        sum_p.2.checked_div(positions.len().try_into().unwrap()).unwrap());

  let mut pos = avg_p;
  let mut max_point = pos;
  let mut field_point = pos;
  let mut max_in_range = 0;
  let mut max_exp : f64 = 0f64;
  let mut map = HashMap::new();
  let mut broad_search = true;
  let mut search_range = 100;

  println!("{:?}", avg_p);
  loop {
    let min : isize = -1 * search_range as isize;
    let max : isize = search_range as isize;
    // println!("working {:?}", pos);

    let range : Vec<isize>;
    if broad_search {
      range = [min,max].to_vec();
    } else {
      range = (min..max).collect();
    }
    for dx in &range {
      for dy in &range {
        for dz in &range {
          let spot = (pos.0+dx,pos.1+dy,pos.2+dz);
          let how_many = nanobots.iter().filter(|(a,r)| manhattan_distance(*a, spot) <= *r).count();
          let experiment : f64 = get_field(spot, &nanobots, &mut map); //  nanobots.iter().map(|(a,r)| { if manhattan_distance(*a, spot) > *r { *r as f64 / manhattan_distance(*a, spot) as f64 } else { 1f64 } } ).sum();

          if how_many > max_in_range {
            max_in_range = how_many;
            max_point = spot;
            // println!("range count {:?} - {}", spot, how_many);
          }

          if experiment > max_exp {
            max_exp = experiment;
            field_point = spot;
            // println!("experiment {:?} - {}", spot, experiment);
          }
        }
      }
    }

    // println!("max in range {}, max exp {}", max_in_range, max_exp);    
    if pos == field_point && broad_search {
      broad_search = false;
      pos = field_point;
      max_point = avg_p;
      field_point = avg_p;
      max_in_range = 0;
      max_exp = 0f64;    
      search_range = 20;
      println!("switching to narrow search");
    } else if pos == max_point && !broad_search {
      println!("found {:?} - {}", pos, manhattan_distance(pos, (0,0,0))); break; 
    } else if broad_search {
      pos = field_point;
    } else {
      pos = max_point;
    }

  }
  let distance = manhattan_distance(max_point, (0,0,0));
  return distance.try_into().unwrap();
}
