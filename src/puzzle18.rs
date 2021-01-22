#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle18_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(1147==super::solve("./inputs/puzzle18-test.txt".to_string()));
  }

  #[test]
  pub fn puzzle18_prod() {
    assert!(common::format_binary(10)=="1010");
    super::solve("./inputs/puzzle18.txt".to_string());
  }
}

use crate::common;
use std::convert::TryInto;
use std::collections::HashMap;

fn get_neighbors(point :(isize,isize)) -> Vec<(isize,isize)> {
  let n = vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];

  return n.iter().map(|p| (p.0+point.0,p.1+point.1)).collect::<Vec<(isize,isize)>>();
}

fn print_map(map : &HashMap<(isize,isize),char>) {
  let max_row = map.keys().map(|(row,_col)| *row).max().unwrap();
  let max_col = map.keys().map(|(_row,col)| *col).max().unwrap();

  for row in 0..=max_row {
    for col in 0..=max_col {
      print!("{}", common::get_spot_on_map(map, row, col, '.'));
    }
    println!();
  }
}

fn one_minute(map : &HashMap<(isize,isize),char>) -> HashMap<(isize,isize),char> {
  let mut new_map = HashMap::new();
  let max_row = map.keys().map(|(row,_col)| *row).max().unwrap();
  let max_col = map.keys().map(|(_row,col)| *col).max().unwrap();

  for row in 0..=max_row {
    for col in 0..=max_col {
      let trees = get_neighbors((row,col)).iter().filter(|(r,c)| common::get_spot_on_map(&map, *r, *c, '.') == '|').count();
      let lumberyard = get_neighbors((row,col)).iter().filter(|(r,c)| common::get_spot_on_map(&map, *r, *c, '.') == '#').count();

      if common::get_spot_on_map(map, row, col, '.') == '.' && trees >= 3 {
        new_map.insert((row,col), '|');
      } else if common::get_spot_on_map(map, row, col, '.') == '|' && lumberyard >= 3 {
        new_map.insert((row,col), '#');
      } else if common::get_spot_on_map(map, row, col, '.') == '|' {
        new_map.insert((row,col), '|');
      } else if common::get_spot_on_map(map, row, col, '.') == '#' && lumberyard >= 1 && trees >= 1 {
        new_map.insert((row,col), '#');
      } else {
        new_map.insert((row,col), '.');
      }

    }
  }
  return new_map;
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);

  let mut map = common::make_map(&lines);

  print_map(&map);
  for i in 0..10 {
    map = one_minute(&map);
    println!("minute {}", i+1);
    print_map(&map);
  }

  let lumberyard = map.values().filter(|c| **c=='#').count();
  let wood = map.values().filter(|c| **c=='|').count();

  println!("lumberyard {}, wood {}, result {}", lumberyard, wood, lumberyard*wood);
  return (lumberyard*wood).try_into().unwrap();
}
