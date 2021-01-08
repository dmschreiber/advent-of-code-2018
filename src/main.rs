pub mod common;
pub mod puzzle1;
pub mod puzzle2;

use tokio_stream::StreamExt;
use tokio::task;
use tokio::runtime;
use tokio::runtime::Builder;
use tokio_util::context::RuntimeExt;

#[macro_use] extern crate lazy_static;
extern crate tokio;

#[tokio::main]
async fn run()  {

    let targets : Vec<(fn(std::string::String) -> i64,String)> = vec![
        (puzzle1::solve,"./inputs/puzzle1.txt".to_string()), // 53 mins
        (puzzle2::solve,"./inputs/puzzle2.txt".to_string()), //24 mins
        ];
    let mut stream = tokio_stream::iter(targets);

    while let Some(t) = stream.next().await {
        tokio::spawn ( async { println!("puzzle is {}", t.0(t.1)); } );

    }

}

fn main() {

    run();
}
