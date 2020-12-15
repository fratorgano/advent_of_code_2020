use std::collections::HashMap;

pub fn game_nth_round(input:&Vec<usize>, n:usize) -> usize{
    let mut number_round = HashMap::new();
    let mut last_spoken = 0;
    
    for i in 0..n{
        last_spoken = if i<input.len(){
            number_round.insert(input[i], i);
            input[i]
        }else{
            let number = match number_round.get(&last_spoken){
                Some(round) =>  i-1-round,
                None => 0
            };
            number_round.insert(last_spoken, i-1);
            number
        };
    }
    last_spoken
}

//Stupid first solution cause I suck at understanding and reading things. I also had to fight with the borrow checker as usual. There's something I keep doing wrong.
/* 
    #[derive(Hash, Eq, PartialEq,Clone)]
    enum Round{
        New(usize),
        Normal(usize)
    }
    
    pub fn game_2020th_round(input:&Vec<usize>) -> usize{

    let mut number_round = HashMap::new();
    let mut last_spoken = 0;
    for i in 0..2020{
        let mut number_round_new = number_round.clone();
        if i<input.len(){
            number_round_new.insert(input[i], Round::New(i+1));
            last_spoken = input[i];
            
        }else{
            if number_round.contains_key(&last_spoken){
                match number_round.get(&last_spoken).unwrap(){
                    Round::New(value) =>  {
                        if *value == i{
                            //println!("{} was spoken first last round, converting",last_spoken);
                            number_round_new.insert(last_spoken, Round::Normal(*value));
                            last_spoken = 0;
                        }else{
                            //println!("{} converting from new to normal so {} - {}",last_spoken, i, value);
                            number_round_new.insert(last_spoken, Round::Normal(i));
                            last_spoken = i-value;
                        }
                        //println!("{} converting from new to normal so {} - {}",last_spoken, i, value);
                    },
                    Round::Normal(value) =>  {
                        //println!("{} is a normal value so {} - {}",last_spoken, i, value);
                        number_round_new.insert(last_spoken, Round::Normal(i));
                        last_spoken = i-value;
                    },
                }
            }else{
                //println!("{} is a completely new value",last_spoken);
                number_round_new.insert(last_spoken, Round::New(i));
                last_spoken = 0;
            }
        }
        number_round = number_round_new;
    }
    last_spoken
} */


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_test() {
        let input = vec![0,3,6];
        assert_eq!(436,game_nth_round(&input, 2020));
    }

    #[test]
    fn day15_test_1() {
        let input = vec![1,3,2];
        assert_eq!(1,game_nth_round(&input, 2020));
    }

    #[test]
    fn day15_test_2() {
        let input = vec![2,1,3];
        assert_eq!(10,game_nth_round(&input, 2020));
    }

    #[test]
    fn day15_test_3() {
        let input = vec![1,2,3];
        assert_eq!(27,game_nth_round(&input, 2020));
    }

    #[test]
    fn day15_test_4() {
        let input = vec![2,3,1];
        assert_eq!(78,game_nth_round(&input, 2020));
    }

    #[test]
    fn day15_test_5() {
        let input = vec![3,2,1];
        assert_eq!(438,game_nth_round(&input, 2020));
    }

    #[test]
    fn day15_test_6() {
        let input = vec![3,1,2];
        assert_eq!(1836,game_nth_round(&input, 2020));
    }

}