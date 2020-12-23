use std::time::Instant;

fn main() {
    let input = vec![6,4,3,7,1,9,2,5,8];

    let start = Instant::now();
    let res1 = day23::cup_ordering(&input,100);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day23::cup_ordering_2(&input, 10_000_000);
    println!("Finished after {:?}", start.elapsed());

    println!("Part 1 result: {}",res1);
    println!("Part 2 result: {}",res2);
}