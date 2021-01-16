#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle14_test() {
    assert!(common::format_binary(10)=="1010");
    assert!(5158916779==super::solve("9".to_string()));
    assert!(0124515891==super::solve("5".to_string()));
    assert!(5941429882==super::solve("2018".to_string()));
    assert!(9==super::solve_part2("51589".to_string()));
    assert!(5==super::solve_part2("01245".to_string()));
    assert!(2018==super::solve_part2("59414".to_string()));
    }

  #[test]
  pub fn puzzle14_prod() {
    assert!(common::format_binary(10)=="1010");
    // println!("{:10}",super::solve("306281".to_string()));
    println!("{:10}",super::solve_part2("306281".to_string()));
  }
}

use crate::common;
use std::convert::TryInto;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug,Clone,PartialEq)]
pub struct Recipe {
  id : usize,
  score : usize,
  next_id : usize,
}

fn get_recipe_number(recipes : &HashMap<usize,Recipe>, current : usize, steps : usize) -> usize {
  let mut current = recipes.get(&current).unwrap();
  for i in 0..steps {
    let next_id = current.next_id;
    current = recipes.get(&next_id).unwrap();
  }
  return current.id;
}

fn add_at_end(recipes : &mut HashMap<usize,Recipe>, sum : usize) {
  let target = *recipes.keys().max().unwrap();
  let mut r = recipes.get_mut(&target).unwrap();
  let last_next_id = r.next_id;
  r.next_id = target+1;
  recipes.insert(target+1, Recipe{ id : target+1, score : sum, next_id : last_next_id});
}

fn print_recipes(recipes : &HashMap<usize,Recipe>, elf1 : usize, elf2 : usize) {
  let mut r = 1;
  loop {
    if r == elf1 {
      print!("({}) ", recipes.get(&r).unwrap().score);
    } else if r == elf2 {
      print!("[{}] ", recipes.get(&r).unwrap().score);
    } else {
      print!("{} ", recipes.get(&r).unwrap().score);
    }
    r = recipes.get(&r).unwrap().next_id;
    if r == 1 { break; }
  }
  println!();
}

fn check_pattern(recipes : &HashMap<usize,Recipe>, pattern : u64, digits : usize) -> bool {
  let retval;
  let start = Instant::now();
  let max = recipes.len();
  // println!("{:?}", start.elapsed());
  if pattern % 10 != recipes.get(&max).unwrap().score as u64 
  { 
    retval= false; 
  } else if (pattern / 10) % 10 != recipes.get(&(max-1)).unwrap().score as u64 {
    retval= false; 

  }
  else if pattern == (0..digits).rev().fold(0, |acc,n| acc*10+recipes.get(&(max-n)).unwrap().score as u64) {
    retval= true;
  } else {
    retval=false;
  }
  // println!("{:?}", start.elapsed());
  return retval;
}

fn solve_part2(input : String) -> i64 {
  let pattern = input.parse::<u64>().unwrap();
  let digits = input.len();

  let mut recipes = HashMap::new();
  let mut elf1_current = 0;
  let mut elf2_current = 1;

  let mut rec_vec = vec![3,7];

  recipes.insert(1,Recipe{ id : 1, score : 3, next_id : 2});
  recipes.insert(2,Recipe{ id : 2, score : 7, next_id : 1});

  loop {
    // print_recipes(&recipes, elf1_current, elf2_current);
    if recipes.len() % 1000 == 0 { println!("{}", recipes.len());}
    let elf1_score = rec_vec[elf1_current];
    let elf2_score = rec_vec[elf2_current];
    let elf1_score = recipes.get(&elf1_current).unwrap().score;
    let elf2_score = recipes.get(&elf2_current).unwrap().score;
    let sum = elf1_score + elf2_score;
    if sum < 10 {
      add_at_end(&mut recipes, sum);
    } else {
      add_at_end(&mut recipes, sum / 10 );
      add_at_end(&mut recipes, sum % 10);
    }
    elf1_current = get_recipe_number(&recipes, elf1_current, elf1_score + 1);
    elf2_current = get_recipe_number(&recipes, elf2_current, elf2_score + 1);

    if recipes.len() > digits && check_pattern(&recipes, pattern, digits) {
      break;
    }
  }
  // print_recipes(&recipes, elf1_current, elf2_current);
  return (recipes.len() - digits )as i64;
  

}

pub fn solve(input : String) -> i64 {
  let recipe_count = input.parse::<usize>().unwrap();

  let mut recipes = HashMap::new();
  let mut elf1_current = 1;
  let mut elf2_current = 2;

  recipes.insert(1,Recipe{ id : 1, score : 3, next_id : 2});
  recipes.insert(2,Recipe{ id : 2, score : 7, next_id : 1});

  while recipes.len() <= recipe_count + 10 {
    // print_recipes(&recipes, elf1_current, elf2_current);
    if recipes.len() % 1000 == 0 { println!("{}", recipes.len());}
    let elf1_score = recipes.get(&elf1_current).unwrap().score;
    let elf2_score = recipes.get(&elf2_current).unwrap().score;
    let sum = elf1_score + elf2_score;
    if sum < 10 {
      add_at_end(&mut recipes, sum);
    } else {
      add_at_end(&mut recipes, sum / 10 );
      add_at_end(&mut recipes, sum % 10);
    }
    elf1_current = get_recipe_number(&recipes, elf1_current, elf1_score + 1);
    elf2_current = get_recipe_number(&recipes, elf2_current, elf2_score + 1);
  }
  

  let mut retval = vec![];
  for i in recipe_count+1..recipe_count+10+1 {
    retval.push(recipes.get(&i).unwrap().score);
  }
  // println!("{:?}", retval);
  return retval.iter().fold(0, |acc,n| acc*10+*n as i64);
  // 3718110721
}
