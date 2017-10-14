extern crate netflix_ratings_exporter;

use std::env;
use std::process;
use netflix_ratings_exporter::{Config, ConfigResult};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::new();
    let config_result = config.from_args(&args);

    match config_result {
        ConfigResult::HelpMenu => {
            print_help();
            process::exit(0);
        },
        ConfigResult::Err(error) => {
            println!("{}\n", error);
            print_help();
            process::exit(1);
        },
        ConfigResult::Ok => ()
    };
}

fn print_help(){
    println!("nre.exe Usage:\n");
    println!("{:10} {}", "-i", "Ignore removed movies");
    println!("{:10} {}", "-h", "Print help menu");
}
