#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle8_test() {
    assert!(common::format_binary(10)=="1010");
    // println!("{}",super::solve("./inputs/puzzle8-test.txt".to_string()));
    println!("{:?}", super::sum_metadata(vec![2,3,0,3,10,11,12,1,2,1,1,0,1,1,1,99,2,1,1,2]));
    println!("{:?}", super::sum_metadata(vec![2,3,1,3,0,1,100,10,11,12,1,1,0,1,99,2,1,1,2]));
    assert!(138==super::solve_part1(&"./inputs/puzzle8-test.txt".to_string()));
    println!("Part 1 result {}", super::solve("./inputs/puzzle8-test.txt".to_string()));
    assert!(66==super::solve_part2(&"./inputs/puzzle8-test.txt".to_string()));
    println!("{:?}", super::sum_value(vec![2,3,1,3,0,1,100,10,11,12,1,1,0,1,99,2,1,1,2]));
  }

  #[test]
  pub fn puzzle8_prod() {
    assert!(common::format_binary(10)=="1010");
    println!("Part 1 result {}", super::solve("./inputs/puzzle8.txt".to_string()));
    assert!(49602==super::solve_part1(&"./inputs/puzzle8.txt".to_string()));
    println!("Part 2 result {}", super::solve_part2(&"./inputs/puzzle8.txt".to_string()));
  }
}

use crate::common;

// part 1 was moving through the list of numbers rather than looping through the number
//   of segments
// part 2 wasn't calling the part 2 method; entered the same value as from part 1

fn sum_value(nums : Vec<u32>) -> (usize,i64) {
  if nums.len() == 0 {
    // println!("Zero len array");
    return (0,0);
  } else if nums[0] == 0 {
    // println!("only meta data {:?}", nums[2..2+nums[1] as usize].to_vec());      
    return (2 + nums[1] as usize, nums[2..2+nums[1] as usize].iter().map(|n| *n as i64).sum::<i64>());
  } else {
    let mut offset : usize = 2;
    if nums.len() <= nums[1] as usize { panic!("length vs metadat {}, {}", nums.len(), nums[1]); }
    let end = nums.len() - nums[1] as usize;
    
    let mut nodes = vec![0; nums[0] as usize];
    for i in 0..nums[0] {
        // println!("Calling sum_metadata {} {} - running total {} {:?}", offset, end, intermediate_total, nums);      
        let (consumed_offset,contributing_total) = sum_value(nums[offset..end].to_vec());
        nodes[i as usize] = contributing_total;

        offset  = offset+consumed_offset;

    }
    let mut intermediate_total = 0;

    for i in nums[offset..offset+nums[1] as usize].to_vec() {
      if let Some(result) = nodes.get((i-1) as usize) {
        intermediate_total = intermediate_total + result;
      }
    }
    return (offset + nums[1] as usize,intermediate_total);
  }
}

fn sum_metadata(nums : Vec<u32>) -> (usize,i64) {
  if nums.len() == 0 {
    // println!("Zero len array");
    return (0,0);
  } else if nums[0] == 0 {
    // println!("only meta data {:?}", nums[2..2+nums[1] as usize].to_vec());      
    return (2 + nums[1] as usize, nums[2..2+nums[1] as usize].iter().map(|n| *n as i64).sum::<i64>());
  } else {
    let mut intermediate_total = 0;
    let mut offset : usize = 2;
    if nums.len() <= nums[1] as usize { panic!("length vs metadat {}, {}", nums.len(), nums[1]); }
    let end = nums.len() - nums[1] as usize;
    
    for _i in 0..nums[0] {
        let (consumed_offset,contributing_total) = sum_metadata(nums[offset..end].to_vec());

        intermediate_total = intermediate_total + contributing_total;
        offset  = offset+consumed_offset;

    }

    return (offset + nums[1] as usize,intermediate_total + nums[offset..offset+nums[1] as usize].iter().map(|n| *n as i64).sum::<i64>());
    // 43152 is too low
  }
}

pub fn solve_part2(file_name : &String) -> i64 {
  let lines = common::read_input(file_name.to_string());

  let nums = lines[0].split(' ').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
  println!("Start solve part 2 with {} nums", nums.len());

  let (used,total) = sum_value(nums);
  println!("Part 2 {} used sum is {}", used, total);
  return total;

}

pub fn solve_part1(file_name : &String) -> i64 {
  let lines = common::read_input(file_name.to_string());

  let nums = lines[0].split(' ').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
  println!("Start solve with {} nums", nums.len());

  let (used,total) = sum_metadata(nums);
  println!("{} used sum is {}", used, total);
  return total;

}
pub fn solve(file_name : String) -> i64 {
  solve_part1(&file_name);
  return solve_part2(&file_name);  
}
