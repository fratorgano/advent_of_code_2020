
pub fn calc_ticket_scanning_error_rate(input:&String, rules:&Vec<(usize,usize)>) -> usize{
    let mut sum_invalid = 0;
    for l in input.lines(){
        let mut sum_ticket = 0;
        for number in l.split(","){
            let n:usize = number.parse().unwrap();
            let mut errors = 0;
            for rule in rules{
                if n<rule.0 || n>rule.1{
                    errors+=1;
                }
                if errors == rules.len(){
                    sum_ticket+=n;
                }
            }
        }
        sum_invalid+=sum_ticket;
    }
    sum_invalid
}

pub fn identify_positions_multiply_ticket(input:&String, rules:&Vec<(usize,usize)>, ticket:&Vec<usize>) -> usize{
    let mut tickets:Vec<Vec<usize>> = vec![];
    for l in input.lines(){
        let ticket:Vec<usize> = l.split(",").map(|number| number.parse::<usize>().unwrap()).collect();
        tickets.push(ticket);
    }
    let mut valid_tickets = vec![];
    for ticket in tickets.iter(){
        let mut correct = true;
        for v in ticket{
            let mut errors = 0;
            for rule in rules{
                if v<&rule.0 || v>&rule.1{
                    errors+=1;
                }
            }
            if errors == rules.len(){
                correct = false;
                break;
            }
        }
        if correct{
            valid_tickets.push(ticket.clone())
        }
    }
    // each array in values shows which rules are supported by values(indexes)
    let mut values:Vec<Vec<usize>> = vec![vec![]; rules.len()];
    for i in (0..rules.len()).step_by(2){
        let mut valids = vec![0;rules.len()];
        for ticket in valid_tickets.iter(){
            for (j,v) in ticket.iter().enumerate(){
                if (v>=&rules[i].0 && v<=&rules[i].1) || (v>=&rules[i+1].0 && v<=&rules[i+1].1) {
                    valids[j]+=1;
                }
            }
        }
        for (j,v) in valids.iter().enumerate(){
            if *v == valid_tickets.len(){
                values[j].push(i/2);
            }
        }
    }
    let mut answer = 1;

    let mut new_values = values.clone();
    for _j in 0..values.len(){
        for i in 0..values.len(){
            let vec = &values[i];
            if vec.len()==1{
                //println!("Rule {} is value {}",vec[0],i);
                if vec[0]<=5{
                    answer*=ticket[i];
                }
                for vec_new in new_values.iter_mut(){
                    vec_new.retain(|&x| x!=vec[0])
                }
                values = new_values.clone();
                break;
            }
        }
    }
    answer
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_test() {
        let rules = vec![(1,3),(5,7),(6,11),(33,44),(13,40),(45,50)];
        let input = String::from("7,3,47
40,4,50
55,2,20
38,6,12");
        assert_eq!(71,calc_ticket_scanning_error_rate(&input, &rules));

    }

    #[test]
    fn day16_test_1() {
        let rules = vec![(0,1),(4,19),(0,5),(8,19),(0,13),(16,19)];
        let ticket = vec![223,139,211,131,113,197,151,193,127,53,89,167,227,79,163,199,191,83,137,149];
        let input = String::from("3,9,18
15,1,5
5,14,9");
        assert_eq!(6540367,identify_positions_multiply_ticket(&input, &rules,&ticket));

    }

}