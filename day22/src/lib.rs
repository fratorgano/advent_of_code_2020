use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

fn parse_hands(input:&String) -> (Vec<u8>,Vec<u8>){
    let mut hands:Vec<&str> = input.split("Player 2:\n").collect();
    hands[0] = hands[0].strip_prefix("Player 1:\n").unwrap().trim();
    hands[1] = hands[1].trim();
    let mut p1 = vec![];
    let mut p2 = vec![];
    for l in hands[0].lines(){
        p1.push(l.parse().unwrap());
    }
    for l in hands[1].lines(){
        p2.push(l.parse().unwrap());
    }
    p1.reverse();
    p2.reverse();
    (p1,p2)
}

pub fn space_card_combat(input:&String) -> usize{
    let (mut p1,mut p2) = parse_hands(input);
    while !p1.is_empty() && !p2.is_empty(){
        //println!("Deck 1: {:?}",p1);
        //println!("Deck 2: {:?}",p2);
        let card_p1 = p1.pop().unwrap();
        let card_p2 = p2.pop().unwrap();
        //println!("P1 plays: {}",card_p1);
        //println!("P2 plays: {}",card_p2);
        match card_p1>card_p2 {
            true => {
                //println!("P1 wins");
                p1.insert(0,card_p2);
                p1.insert(1,card_p1);
                //println!("{:?}",p1);
            },
            false => {
                //println!("P2 wins");
                p2.insert(0,card_p1);
                p2.insert(1,card_p2);
                //println!("{:?}",p2);
            }
        }
    }
    let winner;
    let mut score = 0;
    if p1.is_empty() {winner = p2} else {winner = p1}
    //println!("{:?}",winner);
    for (i,v) in winner.iter().enumerate(){
        score += *v as usize*(i+1);
    }
    score
}

pub fn recursive_space_card_combat(input:&String) -> usize{
    let (mut p1,mut p2) = parse_hands(input);
    let mut hist = HashSet::new();
    while !p1.is_empty() && !p2.is_empty(){
        let winner = recursive_round(&mut p1, &mut p2,&mut hist);
        let card_p1 = winner.0;
        let card_p2 = winner.1;
        match winner.2 {
            1 => {
                //println!("P1 wins");
                /* let mut add = vec!(card_p2,card_p1);
                add.append(&mut p1);
                p1 = add; */
                p1.insert(0,card_p2);
                p1.insert(1,card_p1);
                //println!("{:?}",p1);
            },
            2 => {
                //println!("P2 wins");
                /* let mut add = vec!(card_p1,card_p2);
                add.append(&mut p2);
                p2 = add; */
                p2.insert(0,card_p1);
                p2.insert(1,card_p2);
                //println!("{:?}",p2);
            },
            3 => {
                p2 = vec![];
                break;
            },
            _ => unreachable!()
        }
    }
    let winner;
    let mut score = 0;
    if p1.is_empty() {winner = p2} else {winner = p1}
    for (i,v) in winner.iter().enumerate(){
        score += *v as usize *(i+1);
    }
    score
}


fn recursive_round(p1:&mut Vec<u8>, p2:&mut Vec<u8>, hist:&mut HashSet<u64>) -> (u8,u8,u8){
    let h64 = hash((&p1,&p2));
    if hist.contains(&h64){ return (1,1,3) } else { hist.insert(h64) };

    //println!("ROUND: {}",hist.len());
    //println!("Deck 1: {:?}",p1);
    //println!("Deck 2: {:?}\n",p2);

    let card_p1 = p1.pop().unwrap();
    let card_p2 = p2.pop().unwrap();

    if p1.len()>=card_p1 as usize && p2.len()>=card_p2 as usize{

        let mut rec_p1:Vec<u8> = p1[p1.len()-card_p1 as usize..p1.len()].to_vec();
        let mut rec_p2:Vec<u8> = p2[p2.len()-card_p2 as usize..p2.len()].to_vec();

        //If P1 has the highest card he can't lose subgames
        if rec_p1.iter().max()>rec_p2.iter().max(){
            return (card_p1,card_p2,1)
        }
        
        
        //println!("Starting inner game\n");
        //println!("Inner Deck 1: {:?} ({})",rec_p1, !rec_p1.is_empty());
        //println!("Inner Deck 2: {:?} ({})\n",rec_p2, !rec_p2.is_empty());
        //println!("Ciclo? {}",!rec_p1.is_empty() && !rec_p2.is_empty());
        let mut winner = (0,0,0);
        let mut hist2 = HashSet::new();
        while !rec_p1.is_empty() && !rec_p2.is_empty(){
            winner = recursive_round(&mut rec_p1, &mut rec_p2, &mut hist2);
            let card_p1 = winner.0;
            let card_p2 = winner.1;
            match winner.2 {
                1 => {
                    //println!("P1 wins");
                    /* let mut add = vec!(card_p2,card_p1);
                    add.append(&mut rec_p1);
                    rec_p1 = add; */
                    rec_p1.insert(0,card_p2);
                    rec_p1.insert(1,card_p1);
                    //println!("{:?}",p1);
                },
                2 => {
                    //println!("P2 wins");
                    /* let mut add = vec!(card_p1,card_p2);
                    add.append(&mut rec_p2);
                    rec_p2 = add; */
                    rec_p2.insert(0,card_p1);
                    rec_p2.insert(1,card_p2);
                    //println!("{:?}",p2);
                },
                3 => {
                    winner = (1,1,1);
                    break;
                },
                _ => unreachable!()
            }
            //println!("Inner Deck 1: {:?} ({})",rec_p1, !rec_p1.is_empty());
            //println!("Inner Deck 2: {:?} ({})\n",rec_p2, !rec_p2.is_empty());
        }
        //println!("{} wins inner game\n", winner.2);
        (card_p1,card_p2,winner.2)
    }else if card_p1>card_p2{
        (card_p1,card_p2,1)
    }else{
        (card_p1,card_p2,2)
    }
}

fn hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}




#[cfg(test)]
mod tests {
    use super::*;
    //use std::fs;

    #[test]
    fn day22_test() {
        let input = String::from("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10");
        assert_eq!(306,space_card_combat(&input));
    }

    #[test]
    fn day22_test_2() {
        let input = String::from("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10");
    assert_eq!(291,recursive_space_card_combat(&input));

    }
    /* #[test]
    fn day22_test_input(){
        let input = fs::read_to_string("/home/fra/Rust/advent_of_code_2020/day22/input22.txt").expect("Something went wrong reading the file");
        space_card_combat(&input);
    } */
}