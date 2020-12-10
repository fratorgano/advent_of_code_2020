use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let input = &read_file_args();

    let start = Instant::now();
    let res1 = day03::count_trees(input,3,1);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day03::multiply_trees(input, [(1,1),(3,1),(5,1),(7,1),(1,2)].to_vec());
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
