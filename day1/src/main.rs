use std::env;
use std::fs;
use day1;

fn main() {
    //println!("{:?}",string_to_vec_int32(&read_file_args()));
    let input = string_to_vec_int32(&read_file_args());

    let res1 = day1::find2020_multiply(&input);
    let res2 = day1::find2020_three_multiply(&input);
    
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

fn string_to_vec_int32(s: &String) -> Vec<i32>{
    let numbers: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    return numbers;
}
