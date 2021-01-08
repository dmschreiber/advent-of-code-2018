pub mod common;
pub mod puzzle1;

#[macro_use] extern crate lazy_static;

fn main() {
    println!("puzzle 1 is {}", puzzle1::solve());
}
