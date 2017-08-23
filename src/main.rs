extern crate netflix_ratings_exporter;

use std::env;
use std::process;
use netflix_ratings_exporter::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments {}", err);
        process::exit(1);
    });

    println!("Value: {}", config.ignore_removed_movies);

}

fn print_help(){
    println!("Usage:
-i
    Ignore 'Movie' entries");
}
