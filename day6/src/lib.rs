use std::collections::HashSet;
use std::collections::HashMap;

pub fn count_all_answers(input: &String) -> usize{
    let mut total_answers = 0;
    let mut answers = HashSet::new();
    for l in input.lines(){
        if l.len()!=0 {
            for c in l.chars(){
                answers.insert(c);
            }
        }else{
            total_answers+=answers.len();
            answers.clear();
        }
    }
    total_answers
}

pub fn count_same_answers(input: &String) -> usize{
    let mut total_answers = 0;
    let mut answers = HashMap::new();
    let mut people = 0;
    for l in input.lines(){
        if l.len()!=0 {
            people+=1;
            for c in l.chars(){
                if answers.contains_key(&c){ answers.insert(c, answers.get(&c).unwrap()+1); } else { answers.insert(c, 1);}
            }
        }else{
            for (_,times) in &answers{
                if *times == people{ total_answers+=1 }
            }
            people = 0;
            answers.clear();
        }
    }
    total_answers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1(){
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b

";
        assert_eq!(11,count_all_answers(&String::from(input)));
    }

    #[test]
    fn test_example2(){
        let input = "abc

b
c

ab
ac
        
a
a
a
a

b

";
        assert_eq!(10,count_all_answers(&String::from(input)));
    }

    #[test]
    fn test_example3(){
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b

";
        assert_eq!(6,count_same_answers(&String::from(input)));
    }
}