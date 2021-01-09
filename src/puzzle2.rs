#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle2_test() {
    assert!((0,0)==super::check_id("abcdef".to_string()));
    assert!((1,1)==super::check_id("bababc".to_string()));
    assert!((1,0)==super::check_id("abbcde".to_string()));
    assert!((0,1)==super::check_id("abcccd".to_string()));

    assert!(super::compare(&"abcdef".to_string(),&"azcdef".to_string())==Some("acdef".to_string()));
    assert!(super::compare(&"abcggf".to_string(),&"azcdef".to_string())==None);
  }

  #[test]
  pub fn puzzle2_prod() {
    super::solve("./inputs/puzzle2.txt".to_string());
  }
}

use crate::common;

pub fn check_id(id : String) -> (i64,i64) {
  let mut has_two = 0;
  let mut has_three = 0;

  for c in id.as_bytes() {
    if id.as_bytes().iter().filter(|b| *b==c).count() == 2 { has_two = 1; }
    if id.as_bytes().iter().filter(|b| *b==c).count() == 3 { has_three = 1; }
  }

  return (has_two,has_three);
}

pub fn compare(id1 : &String, id2 :&String) -> Option<String> {
  let mut different = 0;
  let mut result = vec![];

  for (index,c) in id1.as_bytes().iter().enumerate() {    
    if c == id2.as_bytes().get(index).unwrap() {
      result.push(*c);
      // same
    } else {
      different += 1;
      if different > 1 { return None; }
    }
  }
  if different == 1 {
    return Some(result.iter().map(|c| *c as char).collect::<String>());
  } else {
    return None;
  }
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");
  
  let v = lines.iter().map(|l| check_id(l.to_string())).collect::<Vec<(i64,i64)>>();
  let retval = v.iter().map(|(a,_b)| *a).sum::<i64>() * v.iter().map(|(_a,b)| b).sum::<i64>();
  println!("{} checksum", retval);

  for l1 in &lines {
    for l2 in &lines {
      if let Some(r) = compare(&l1, &l2) {
        println!("result is {}", r);
        
      }
    }
  }

  return retval;
}
