use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let mut input = string_to_vec_usize(&read_file_args());

    let start = Instant::now();
    let res1 = day10::adapters_chain_differences(&mut input);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day10::adapters_chain_options(&input);
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

fn string_to_vec_usize(s: &String) -> Vec<usize>{
    let numbers: Vec<usize> = s
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    return numbers;
}
