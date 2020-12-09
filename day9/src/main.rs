use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let input = string_to_vec_usize(&read_file_args());

    let start = Instant::now();
    let res1 = day9::find_wrong_cipher_number(&input,25);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day9::find_encryption_weakness(&input,25);
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
