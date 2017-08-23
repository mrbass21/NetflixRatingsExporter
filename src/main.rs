extern crate netflix_ratings_exporter;

use std::env;
use std::process;
use netflix_ratings_exporter::Config;
use netflix_ratings_exporter::ConfigResult;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config_result = Config::new(&args).unwrap_or_else(|err| {
            println!("{}\n", err);
            print_help();
            process::exit(1);        
    });

    let config: Config = match config_result {
        ConfigResult::HelpMenu => {
            print_help();
            process::exit(0);
        },
        ConfigResult::Result(config) => config,
    };
}

fn print_help(){
    println!("Usage:\n");
    println!("{:10} {}", "-i", "Ignore removed movies");
    println!("{:10} {}", "-h", "Print help menu");
}
