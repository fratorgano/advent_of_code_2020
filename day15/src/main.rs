use std::time::Instant;

fn main() {
    let input:Vec<usize> = vec![9,12,1,4,17,0,18];

    let start = Instant::now();
    let res1 = day15::game_nth_round(&input,2020);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day15::game_nth_round(&input,30000000);
    println!("Finished after {:?}", start.elapsed());

    println!("Part 1 result: {}",res1);
    println!("Part 2 result: {}",res2);
}
