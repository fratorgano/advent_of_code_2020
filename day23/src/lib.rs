//Had some troubles with the example input provided, looked up some help on the adventofcode subreddit

use std::{collections::HashSet};

type T = u32;

fn unwrap_pointers(ps:&[T], cur:T) -> Vec<T>{
    let mut res = Vec::new();
    let mut p = cur;
    for _ in 0..ps.len()+1{
        res.push(p);
        p = ps[p as usize];
        if p == cur {return res;}
    }
    panic!("Loop while unfolding, cur={}:\n 0, 1, 2, 3, 4, 5, 6, 7, 8, 9\n{:?}", cur, &ps[..10]);
}

fn crab_step(pointers: &mut [T],cur:T) -> T{
    let n = pointers.len()-1;

    //remove 3 values
    let t0 = pointers[cur as usize];
    let t1 = pointers[t0 as usize];
    let t2 = pointers[t1 as usize];

    let ts:HashSet<T> = [t0,t1,t2].iter().cloned().collect();

    pointers[cur as usize] = pointers [t2 as usize];

    //find index to add values to
    let mut dst = if cur>1 {cur-1} else {n as T};
    while ts.contains(&dst){
        dst = if dst>1 {dst-1} else {n as T};
    }

    //insert ts
    pointers[t2 as usize] = pointers[dst as usize];
    pointers[dst as usize] = t0;

    pointers[cur as usize]
} 

pub fn cup_ordering(cups: &Vec<T>, times: usize) -> String{
    let n = cups.len();

    let mut pointers:[T;10] = [0;10];
    for w in cups.windows(2) {
        pointers[w[0] as usize] = w[1];
    }
    pointers[cups[n-1] as usize] = cups[0];
    let mut cur = cups[0];

    println!("Input: {:?}, as pointer: {:?}", cups, unwrap_pointers(&pointers, cur));

    for _i in 0..times{
        cur = crab_step(&mut pointers, cur);
    }

    let mut s:String = String::from("");
    for (i,x) in unwrap_pointers(&pointers, 1).iter().enumerate(){
        if i==0{
            continue;
        }
        s.push_str(&x.to_string());
    }
    String::from(format!("{}", s))
}

pub fn cup_ordering_2(cups:&Vec<T>,times:usize) -> usize{
    let n:T = 1_000_000;
    let mut pointers:Vec<T> = (1..(n+2)).collect();
    pointers[n as usize] = cups[0];
    for w in cups.windows(2) {
        pointers[w[0] as usize] = w[1];
    }
    pointers[cups[cups.len()-1] as usize] = (cups.len()+1) as T;
    let mut cur = cups[0];

    for _ in 0..times {
        cur = crab_step(&mut pointers, cur);
    }
    let v = unwrap_pointers(&pointers, 1);
    v[1] as usize * v[2] as usize 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_test() {
        let input = vec![3,8,9,1,2,5,4,6,7];
        assert_eq!("67384529",cup_ordering(&input, 100).trim());
    }

    #[test]
    fn day22_test_2() {
        let input = vec![3,8,9,1,2,5,4,6,7];
        assert_eq!(149245887792,cup_ordering_2(&input, 10_000_000));
    }
}