//Had to lookup some help for the second part, I had some troubles with rust's borrow checker. Ended up borrowing from some of the advent of code subreddit's solutions. 

use std::collections::HashMap;

enum Instruction {
    Mask(usize, usize, usize),
    Mem(usize, usize),
}

pub fn sum_memory(input: &str) -> usize {
    input
        .lines()
        .map(Instruction::from)
        .fold(((0, 0, 0), HashMap::new()), |(mut mask, mut mem), ins| {
            match ins {
                Instruction::Mask(z, o, x) => mask = (z, o, x),
                Instruction::Mem(addr, val) => {
                    mem.insert(addr, val & !mask.0 | mask.1);
                }
            }
            (mask, mem)
        })
        .1
        .values()
        .sum::<usize>()
}

pub fn sum_memory_floating(input: &str) -> usize {
    input
        .lines()
        .map(Instruction::from)
        .fold(((0, 0, 0), HashMap::new()), |(mut mask, mut mem), ins| {
            match ins {
                Instruction::Mask(z, o, x) => mask = (z, o, x),
                Instruction::Mem(addr, val) => {
                    decode(addr, mask.1, mask.2).into_iter().for_each(|a| {
                        mem.insert(a, val);
                    });
                }
            }
            (mask, mem)
        })
        .1
        .values()
        .sum::<usize>()
}

fn decode(addr: usize, o: usize, x: usize) -> Vec<usize> {
    (0..64)
        .filter(|i| (1 << i) & x > 0)
        .fold(vec![addr | o], |acc, i| {
            acc.into_iter()
                .flat_map(|j| vec![j & !(1 << i), j | (1 << i)])
                .collect()
        })
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s.starts_with("mask") {
            let m = ['0', '1', 'X']
                .iter()
                .map(|c| {
                    s.chars()
                        .skip(7)
                        .map(|d| if *c == d { 1 } else { 0 })
                        .fold(0, |acc, x| acc * 2 + x)
                })
                .collect::<Vec<_>>();
            Instruction::Mask(m[0], m[1], m[2])
        } else {
            let mut s = s.strip_prefix("mem[").unwrap().split(']');
            let addr = s.next().unwrap().parse().unwrap();
            let val = s.next().unwrap().split_at(3).1.parse().unwrap();
            Instruction::Mem(addr, val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_test() {
        let input = String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0");
        assert_eq!(165, sum_memory(&input)) ;
    }
    #[test]
    fn day14_test_2() {
        let input = String::from("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1");
        assert_eq!(208, sum_memory_floating(&input)) ;
    }
}