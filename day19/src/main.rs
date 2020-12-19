use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let rules = read_file_args_1();
    let input = read_file_args_2();

    let start = Instant::now();
    let res1 = day19::count_valid(&rules, &input);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day19::count_valid_2(&rules, &input);
    println!("Finished after {:?}", start.elapsed());

    println!("Part 1 result: {}",res1);
    println!("Part 2 result: {}",res2);
}

fn read_file_args_1() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You forgot to add file name to arguments");
    }
    fs::read_to_string(&args[1]).expect("Something went wrong reading the file")
}
fn read_file_args_2() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("You forgot to add file name to arguments");
    }
    fs::read_to_string(&args[2]).expect("Something went wrong reading the file")
}
