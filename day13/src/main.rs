use std::time::Instant;

fn main() {
    let time = 1006401;
    let buses = vec!["17","x","x","x","x","x","x","x","x","x","x","37","x",
                     "x","x","x","x","449","x","x","x","x","x","x","x","23",
                     "x","x","x","x","13","x","x","x","x","x","19","x","x",
                     "x","x","x","x","x","x","x","x","x","607","x","x","x",
                     "x","x","x","x","x","x","41","x","x","x","x","x","x",
                     "x","x","x","x","x","x","x","x","x","x","x","x","29"];

    let start = Instant::now();
    let res1 = day13::find_best_bus(time,&buses);
    println!("Finished after {:?}", start.elapsed());

    let start = Instant::now();
    let res2 = day13::find_best_timestamp(&buses);
    println!("Finished after {:?}", start.elapsed());

    println!("Part 1 result: {}",res1);
    println!("Part 2 result: {}",res2);
}