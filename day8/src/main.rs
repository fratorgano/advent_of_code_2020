use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let input = read_file_args();

    let start = Instant::now();
    let res1 = match day8::run_program(&day8::input_to_instructions(&input)) {
        Ok(accumulator) => accumulator,
        Err(accumulator) => accumulator,
    };
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day8::fix_loop(&mut day8::input_to_instructions(&input));
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