//20/12/2020: Today was way too hard and I couldn't solve it in the time that I had. I will probably come back to it eventually.
//25/12/2020: I did it. I admittedly looked up some other solutions and took inspiration from it.

use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let input = read_file_args();

    let start = Instant::now();
    let res1 = day20::multiply_corner_ids(&input);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day20::count_sharp_no_sea_monster(&input);
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