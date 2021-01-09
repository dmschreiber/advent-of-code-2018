pub mod common;
pub mod puzzle1;
pub mod puzzle2;
pub mod puzzle3;
pub mod puzzle4;
pub mod puzzle5;

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
        (puzzle1::solve,"./inputs/puzzle1.txt".to_string()), // 53 mins
        (puzzle2::solve,"./inputs/puzzle2.txt".to_string()), //24 mins
        (puzzle3::solve,"./inputs/puzzle3.txt".to_string()), // 33 mins
        (puzzle4::solve,"./inputs/puzzle4.txt".to_string()), // try tags (57 mins)
        (puzzle5::solve,"./inputs/puzzle5.txt".to_string()), // 75 mins
        ];
    let mut stream = tokio_stream::iter(targets);

    while let Some(t) = stream.next().await {
        tokio::spawn ( async move { 
            let start = Instant::now();
            println!("puzzle ({}) result is {} in {:?}", &t.1, t.0(t.1.to_string()),start.elapsed()); } );

    }

}
