pub mod common;
pub mod puzzle1;
pub mod puzzle13;
pub mod puzzle12;
pub mod puzzle11;
pub mod puzzle10;
pub mod puzzle9;
pub mod puzzle8;
pub mod puzzle7;
pub mod puzzle2;
pub mod puzzle3;
pub mod puzzle4;
pub mod puzzle5;
pub mod puzzle6;

use tokio_stream::StreamExt;
use std::time::Instant;
// use tokio::task;
// use tokio::runtime;
// use tokio::runtime::Builder;
// use tokio_util::context::RuntimeExt;

#[macro_use] extern crate lazy_static;
extern crate tokio;

#[tokio::main]
async fn main() {


    let targets : Vec<(fn(std::string::String) -> i64,String)> = vec![
        (puzzle1::solve,"./inputs/puzzle1.txt".to_string()),
        (puzzle13::solve,"./inputs/puzzle13.txt".to_string()),
        (puzzle12::solve,"./inputs/puzzle12.txt".to_string()),
        (puzzle11::solve,"./inputs/puzzle11.txt".to_string()),
        (puzzle10::solve,"./inputs/puzzle10.txt".to_string()),
        (puzzle9::solve,"428 players; last marble is worth 70825 points".to_string()),
        (puzzle8::solve,"./inputs/puzzle8.txt".to_string()), // 2+ hrs
        (puzzle7::solve,"./inputs/puzzle7.txt".to_string()), // 2+ hrs
        (puzzle2::solve,"./inputs/puzzle2.txt".to_string()), //24 mins
        (puzzle3::solve,"./inputs/puzzle3.txt".to_string()), // 33 mins
        (puzzle4::solve,"./inputs/puzzle4.txt".to_string()), // try tags (57 mins)
        (puzzle5::solve,"./inputs/puzzle5.txt".to_string()), // 75 mins
        (puzzle6::solve,"./inputs/puzzle6.txt".to_string()), // 66 mins
        ];
    let mut stream = tokio_stream::iter(targets);

    while let Some(t) = stream.next().await {
        tokio::spawn ( async move { 
            let start = Instant::now();
            println!("puzzle ({}) result is {} in {:?}", &t.1, t.0(t.1.to_string()),start.elapsed()); } );

    }

}
