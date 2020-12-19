// These days are starting to become way too hard, since I'm learning a language from now on some solutions will be based on someone else's that I rewrite to learn and understand
// the logic behind it. Today's inspiration was (https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/19.rs)

use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
enum Rule{
    Comb(Vec<Vec<usize>>),
    Char(char)
}

fn parse_rules(input:&String) -> HashMap<usize, Rule>{
    let mut rules = HashMap::new();
    for l in input.lines(){
        let parts:Vec<&str> = l.split(": ").collect();
        let index:usize = parts[0].parse().unwrap();
        if parts[1].contains("\""){
            rules.insert(index, Rule::Char(parts[1].as_bytes()[1] as char));
            continue;
        }
        let v = parts[1].split(" | ")
        .map(|p| p.split_whitespace()
          .map(|i| i.parse::<usize>().unwrap())
          .collect()
        ).collect();
        rules.insert(index, Rule::Comb(v));
    }
    rules
}

fn matches(rules:&HashMap<usize,Rule>, id:usize) -> Vec<String>{
    match rules.get(&id).unwrap(){
        Rule::Char(c) => vec![c.to_string()],
        Rule::Comb(v) =>{
            v.iter().flat_map(|p| match p[..]{
                [p] => matches(rules, p),
                [p1,p2] => matches(rules, p1).iter()
                    .cartesian_product(matches(rules,p2).iter())
                    .map(|(s1,s2)| format!("{}{}",s1,s2))
                    .collect(),
                _ => unreachable!()
            }).collect()
        }

    }
}

fn check_match_p1(prefixes:&[String], suffixes:&[String], s:&str) -> bool{
    prefixes.iter()
        .filter(|&p| s.starts_with(p))
        .any(|p| {
            let s = &s[p.len()..];
            prefixes.iter()
                .cartesian_product(suffixes.iter())
                .filter(|(p1,p2)| s.len() == p1.len() + p2.len())
                .any(|(p1,p2)| s.starts_with(p1) && s.ends_with(p2))
        })
}



fn check_match_p2(prefixes: &[String], suffixes: &[String], s: &str) -> bool {
    prefixes.iter()
      .filter(|&p| s.starts_with(p))
      .map(|p| &s[p.len()..])
      .any(|s|
        check_match_p2(prefixes, suffixes, s)
        || check_rule_11(prefixes, suffixes, s)
    )
}
fn check_rule_11(prefixes: &[String], suffixes: &[String], s: &str) -> bool {
    prefixes.iter()
        .cartesian_product(suffixes.iter())
        .filter(|&(p1,p2)| p1.len() + p2.len() <= s.len())
        .filter(|&(p1,p2)| s.starts_with(p1) && s.ends_with(p2))
        .map(|(p1,p2)| &s[p1.len()..(s.len() - p2.len())])
        .any(|s| s.len() == 0 || check_rule_11(prefixes, suffixes, s))
}


pub fn count_valid(rules:&String, input:&String) -> usize{
    let map_rul = parse_rules(rules);
    let v42 = matches(&map_rul, 42);
    let v31 = matches(&map_rul, 31);
    let mut count = 0;
    for l in input.lines(){
        if check_match_p1(&v42, &v31, &l){
            count+=1;
        }
    }
    count
}
pub fn count_valid_2(rules:&String, input:&String) -> usize{
    let map_rul = parse_rules(rules);
    //I have no idea why we use rule 42 and 31
    let v42 = matches(&map_rul, 42);
    let v31 = matches(&map_rul, 31);
    let mut count = 0;
    for l in input.lines(){
        if check_match_p2(&v42, &v31, &l){
            count+=1;
        }
    }
    count
}