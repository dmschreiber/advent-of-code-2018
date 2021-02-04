#[cfg(test)]
mod tests {
  #[test]
  pub fn puzzle23_test() {
    assert!(((41289914,12552653,-7638886),70344373)==super::parse_pos(&"pos=<41289914,12552653,-7638886>, r=70344373".to_string()));
    assert!(36==super::solve("./inputs/puzzle23-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle23_prod() {
    assert!(116547949==super::solve("./inputs/puzzle23.txt".to_string()));
  }
}

use crate::common;
use std::convert::TryInto;
use regex::Regex;

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

fn get_composite_field(spot : (isize,isize,isize), bots : &Vec<((isize,isize,isize),usize)>) -> f64 {
  let experiment : f64 = bots.iter().map(|(a,r)| { if manhattan_distance(*a, spot) > *r { *r as f64 / manhattan_distance(*a, spot) as f64 } else { 1f64 } } ).sum();
  let how_many = bots.iter().filter(|(a,r)| manhattan_distance(*a, spot) <= *r).count();
  let field = how_many as f64 * 1000f64 + experiment;
  
  return field;

}

fn get_average(positions : &Vec<(isize,isize,isize)>) -> (isize,isize,isize) {
  let sum_p : (isize,isize,isize) = positions.iter().fold((0,0,0),|acc,p| (acc.0+p.0,acc.1+p.1,acc.2+p.2) );
  let avg_p = (sum_p.0.checked_div(positions.len().try_into().unwrap()).unwrap(), 
        sum_p.1.checked_div(positions.len().try_into().unwrap()).unwrap(), 
        sum_p.2.checked_div(positions.len().try_into().unwrap()).unwrap());

  return avg_p;
}

fn solve_part2(nanobots : &Vec<((isize,isize,isize),usize)>) -> (isize,isize,isize) {
  let positions = nanobots.iter().map(|(a,_r)| *a).collect::<Vec<(isize,isize,isize)>>(); 
  let avg_p = get_average(&positions);

  let mut search_range = 4096;
  let mut pos = avg_p;
  
  while search_range >= 1 {
    let min : isize = -1 * search_range as isize;
    let max : isize = search_range as isize;
    
    let range : Vec<isize>;
    range = [min,0,max].to_vec();

    let mut max_field = (0f64,(0,0,0));

    for dx in &range {
      for dy in &range {
        for dz in &range {
          let spot = (pos.0+dx,pos.1+dy,pos.2+dz);
          let field_power : f64 = get_composite_field(spot, &nanobots);
          if field_power > max_field.0 {
            max_field = (field_power,spot);
          }
        }
      }
    }

    if pos == max_field.1 {
      search_range = search_range / 2;
    } else {
      pos = max_field.1;
    }
  }

  return pos;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let nanobots = lines.iter().map(|l| parse_pos(l)).collect::<Vec<((isize,isize,isize),usize)>>();

  let biggest = nanobots.iter().map(|(_a,r)| *r).max().unwrap();
  let pos = nanobots.iter().filter(|(_a,r)| *r==biggest).map(|(a,_r)| *a).collect::<Vec<(isize,isize,isize)>>()[0];

  let in_range = nanobots.iter().filter(|(a,_r)| manhattan_distance(*a, pos) <= biggest).count();
  println!("{:?} - in range {}", pos, in_range);

  let max_point = solve_part2(&nanobots);
  let distance = manhattan_distance(max_point, (0,0,0));

  println!("Max point {:?} at distance {}", max_point, distance);
  return distance.try_into().unwrap();
}
