#[cfg(test)]
mod tests {
  use crate::common;
  #[test]
  pub fn puzzle4_test() {
    // use regex::Regex;
    // assert!(common::format_binary(10)=="1010");
    assert!(4455==super::solve("./inputs/puzzle4-test.txt".to_string()));
    if let Some(inner) = super::GUARD_REGEX.captures("[1518-11-01 00:00] Guard #10 begins shift") {
      assert!(inner[1] == *"1518");
      assert!(inner[2] == *"11");
      assert!(inner[3] == *"01");
      assert!(inner[4] == *"00");
      assert!(inner[5] == *"00");
      assert!(inner[6] == *"10");
    }

    if let Some(inner) = super::GUARD_REGEX.captures("[1518-11-01 00:05] falls asleep") {
      assert!(inner[1] == *"1518");
      assert!(inner[2] == *"11");
      assert!(inner[3] == *"01");
      assert!(inner[4] == *"00");
      assert!(inner[5] == *"05");
    }
   
    if let Some(inner) = super::GUARD_REGEX.captures("[1518-11-01 00:25] wakes up") {
      assert!(inner[1] == *"1518");
      assert!(inner[2] == *"11");
      assert!(inner[3] == *"01");
      assert!(inner[4] == *"00");
      assert!(inner[5] == *"25");
    }

  }

  #[test]
  pub fn puzzle4_prod() {
    assert!(common::format_binary(10)=="1010");
    assert!( 56901==super::solve("./inputs/puzzle4.txt".to_string()));
  }
}

use crate::common;
use std::convert::TryInto;
use std::collections::HashMap;
use regex::Regex;

////////
// RegEx
lazy_static! {
  static ref GUARD_REGEX: Regex = Regex::new(r"^\[([0-9+]+)-([0-9+]+)-([0-9+]+) ([0-9+]+):([0-9+]+)\] Guard #([0-9+]+) (.*)$").unwrap();
  static ref ASLEEP_REGEX: Regex = Regex::new(r"^\[([0-9+]+)-([0-9+]+)-([0-9+]+) ([0-9+]+):([0-9+]+)\] falls asleep$").unwrap();
  static ref WAKES_REGEX: Regex = Regex::new(r"^\[([0-9+]+)-([0-9+]+)-([0-9+]+) ([0-9+]+):([0-9+]+)\] wakes up$").unwrap();
}

#[derive(Debug,Clone)]
pub enum Action {
  Begin,
  Sleep,
  Wake
}

#[derive(Debug,Clone)]
pub struct Record {
  timestamp : String,
  id : u32,
  action : Action,
}

pub struct RangeRec {
  id : u32,
  start : u64,
  end : u64,
  duration : u64
}

pub fn solve(file_name : String) -> i64 {
  let lines = common::read_input(file_name);
  println!("Start solve");

  let mut records = vec![];
  for l in &lines {
    if let Some(inner) = GUARD_REGEX.captures(&l) {
      let timestamp = format!("{}{}{}{}{}", inner[1].to_string(), inner[2].to_string(), inner[3].to_string(), inner[4].to_string(), inner[5].to_string()).to_string();
      let id = inner[6].parse::<u32>().unwrap();

      records.push(Record { timestamp: timestamp, id : id, action : Action::Begin});
    } else     if let Some(inner) = ASLEEP_REGEX.captures(&l) {
      let timestamp = format!("{}{}{}{}{}", inner[1].to_string(), inner[2].to_string(), inner[3].to_string(), inner[4].to_string(), inner[5].to_string()).to_string();
      let id = 0;

      records.push(Record { timestamp: timestamp, id : id, action : Action::Sleep});

    } else     if let Some(inner) = WAKES_REGEX.captures(&l) {
      let timestamp = format!("{}{}{}{}{}", inner[1].to_string(), inner[2].to_string(), inner[3].to_string(), inner[4].to_string(), inner[5].to_string()).to_string();
      let id = 0;

      records.push(Record { timestamp: timestamp, id : id, action : Action::Wake});

    }
  }
  
  records.sort_by(|a,b| a.timestamp.cmp(&b.timestamp));

  let mut last_id = 0;
  let mut start_sleep : u64 = 0;
  let mut duration;

  let mut range_rec = vec![];

  for r in records  {
    match r.action {
      Action::Begin  => { last_id = r.id;  }
      Action::Sleep => { start_sleep = r.timestamp.parse::<u64>().unwrap(); }
      Action::Wake => { 
        duration = r.timestamp.parse::<u64>().unwrap() - start_sleep; 
        range_rec.push( RangeRec{ id : last_id, start : start_sleep, end : r.timestamp.parse::<u64>().unwrap(), duration : duration });
      }
    }
  }

  let mut biggest = 0;
  let mut which_guard = 0;
  for r in &range_rec {
    if range_rec.iter().filter(|i| i.id == r.id).map(|i| i.duration).sum::<u64>() > biggest {
      biggest = range_rec.iter().filter(|i| i.id == r.id).map(|i| i.duration).sum::<u64>();
      which_guard = r.id;
    }
  }

  let guard_id = which_guard;

  let mut time_map = HashMap::new();
  for r in &range_rec {
    if r.id == guard_id {
      let start = r.start % 100;
      let end = r.end % 100;
      for m in start..end {
        if let Some(total) = time_map.get_mut(&m) {
          *total += 1;
        } else {
          time_map.insert(m,1);
        }
      }
    }
  }

  let minute = time_map.keys().filter(|k| time_map.get(k).unwrap() == time_map.values().max().unwrap()).map(|k| *k).collect::<Vec<u64>>()[0];

  println!("part 1 Guard id {} and minutes {} answer {}", guard_id, minute, guard_id as u64 * minute);


  let mut all_time_map = HashMap::new();
  for r in &range_rec {
    let start = r.start % 100;
    let end = r.end % 100;
    for m in start..end {
      if let Some(total) = all_time_map.get_mut(&(r.id,m)) {
        *total += 1;
      } else {
        all_time_map.insert((r.id,m),1);
      }
    }
  }

  let guard_minute = all_time_map.keys().filter(|k| all_time_map.get(k).unwrap() == all_time_map.values().max().unwrap()).map(|k| *k).collect::<Vec<(u32,u64)>>()[0];

  println!("part 2 guard {}, minute {}, answer {}", guard_minute.0, guard_minute.1, guard_minute.0 as u64*guard_minute.1);
  return (guard_minute.0 as u64*guard_minute.1).try_into().unwrap();
}
