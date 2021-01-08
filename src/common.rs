use std::fs;
use std::collections::HashMap;
use regex::Regex;


#[cfg(test)]
mod tests {
  #[test]
  pub fn common_tests() {
    assert!(super::format_binary(10)=="1010");
    println!("{}",super::format_binary_len(10, 8));
    assert!(super::format_binary_len(10,8)=="00001010");
    assert!(super::parse_binary(&"1010".to_string(), '1', '0')==10);
    assert!(super::get_number_between_text("some 10 thing".to_string())==10);
  }
}

pub fn read_input(filename: String) -> Vec<String> {

  let contents = fs::read_to_string(filename)
  .expect("Something went wrong reading the file");

  let lines: Vec<String> = contents.split("\n\n").map(|s| (&*s).to_string() ).collect();

  lines
}

////////
// RegEx
lazy_static! {
  static ref NUMBER_REGEX: Regex = Regex::new(r"^(.*) ([0-9*+]+)(.*)$").unwrap();
}

// gets a number on a line with stuff before & after
pub fn get_number_between_text(expression : String) -> i64 {

  if let Some(inner) = NUMBER_REGEX.captures(&expression) {
    let r = inner[2].to_string();
    println!("{}", r);
    let n = r.parse::<i64>();
    return n.unwrap();
  }
  panic!("not a number");
}
//////// Map Functions 
// 

pub fn get_spot_on_map(map : &HashMap<(isize,isize),char>, row : isize, col : isize, default : char) -> char {
    if let Some(b) = map.get( &( row, col ) ) {
      return *b;
    } else {
      return default;
    }
}
pub fn get_min_row(map : HashMap<(isize,isize),char> ) -> isize {
  return map.keys().map(|(r,_c)| *r).min().unwrap();
}

pub fn get_max_row(map : HashMap<(isize,isize),char> ) -> isize {
  return map.keys().map(|(r,_c)| *r).max().unwrap();
}

pub fn make_map(lines : &Vec<String>) -> HashMap<(isize,isize),char> {
  let mut map = HashMap::new();

  for (row,line) in lines.iter().enumerate() {
    for (col,value) in line.as_bytes().iter().enumerate() {
      map.insert((row as isize,col as isize),*value as char);
    }
  }
  return map;
}

// format a dec as binary string
pub fn format_binary(num : u32) -> String {
  return format!("{:b}", num);
}

pub fn format_binary_len(num : u32, len : usize) -> String {
  return format!("{:0len$b}", num, len=len);
}

pub fn format_custom_binary(num : u32, one_char : char, zero_char : char) -> String {
  let mut b_num = format!("{:b}", num);
  b_num = b_num.replace("1", &one_char.to_string());
  b_num = b_num.replace("0", &zero_char.to_string());
  return b_num;
}

// parse a binary string to decimal
// use parse_binary("##..#.#", "#", ".") -> dec of "1100101"
pub fn parse_binary(bin_str : &String, one_char : char, zero_char : char) -> u32 {
  return u32::from_str_radix(&bin_str.replace(&one_char.to_string(),"1").replace(&zero_char.to_string(),"0"),2).unwrap();
}