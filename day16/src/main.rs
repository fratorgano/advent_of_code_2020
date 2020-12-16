use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let input = read_file_args();
    let rules = vec![(39,715),(734,949),(30,152),(160,959),(34,780),(798,955),(32,674),(699,952),(38,55),(74,952),(45,533),(547,970),(27,168),(191,969),
                     (43,585),(599,953),(40,831),(837,961),(37,293),(301,974),(40,89),(112,950),(25,600),(625,970),(45,318),(341,954),(40,898),(912,968),
                     (38,872),(881,958),(37,821),(831,958),(26,343),(365,956),(37,857),(872,960),(36,425),(445,972),(44,270),(286,967)
                    ];
    let ticket = vec![223,139,211,131,113,197,151,193,127,53,89,167,227,79,163,199,191,83,137,149];

    let start = Instant::now();
    let res1 = day16::calc_ticket_scanning_error_rate(&input,&rules);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day16::identify_positions_multiply_ticket(&input,&rules,&ticket);
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
