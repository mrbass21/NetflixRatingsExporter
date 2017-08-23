extern crate netflix_ratings_exporter;

use std::env;
use std::process;
use netflix_ratings_exporter::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {

        if(err != "Help Menu"){
            println!("{}\n", err);
            print_help();
            process::exit(1);
        } else {
            print_help();
            process::exit(0);
        }
        
        
    });


}

fn print_help(){
    println!("Usage:\n");
    println!("{:10} {}", "-i", "Ignore removed movies");
    println!("{:10} {}", "-h", "Print help menu");
}
