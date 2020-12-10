//Please don't even bother to look at this code, it's a real mess, I hate parsing inputs with Rust

use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let input = &read_file_args();

    let start = Instant::now();
    let res1 = day04::check_passport_validity(input);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day04::check_passport_validity_2(input);
    println!("Finished after {:?}", start.elapsed());

    println!("Part 1 result: {}",res1);
    println!("Part 2 result: {}",res2);
}

fn read_file_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You forgot to add file name to arguments");
    }
    fs::read_to_string(&args[1]).expect("Something went wrong reading the file")
}
