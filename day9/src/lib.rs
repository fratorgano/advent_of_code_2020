// I tried to work with indexes only and it actually worked! (had a couple of off by one but was mostly fine)


pub fn find_wrong_cipher_number(numbers:&Vec<usize>, preamble_length: usize) -> usize{
    let mut found = false;
    for i in preamble_length..numbers.len(){
        for j in i-preamble_length..i{
            for k in i-preamble_length..i{
                    if numbers[j]!=numbers[k]{
                    //println!("{}({})+{}({})={} == {}({})",numbers[j],j,numbers[k],k,numbers[j]+numbers[k],numbers[i],i);
                    if numbers[j]+numbers[k]==numbers[i]{
                        found = true;
                        break;
                    }
                }
                if found {break};
            }
            if found {break};
        }
        if !found {return numbers[i]} else {found = false;};
    }
    0
}

pub fn find_encryption_weakness(numbers:&Vec<usize>, preamble_length: usize) -> usize{
    let wrong = find_wrong_cipher_number(numbers, preamble_length);
    let mut sum = 0;
    let mut found = false;
    let mut start = 0;
    let mut end = 0;
    for i in 0..numbers.len(){
        //if numbers[i]>wrong {break}
        sum+=numbers[i];
        for j in i+1..numbers.len(){
            if numbers[j]>wrong {break}
            sum+=numbers[j];
            if sum>wrong{break;}
            //println!("range: {}-{}, sum: {}", i,j,sum);
            if sum == wrong{
                start = i;
                end = j;
                found=true;
                //println!("range: {}-{}, sum: {}", i,j,sum);
                break;
            }
        }
        if !found{
            sum=0;
        }else {break;}
    }
    if !found{
        panic!("Error, range with sum {} not found", wrong);
    }
    //println!("{}-{}{:?}",start,end,&numbers[start..=end]);
    numbers[start..=end].iter().min().unwrap() + numbers[start..=end].iter().max().unwrap()
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day09_test() {
        assert_eq!(127, find_wrong_cipher_number(&vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576], 5));
    }

    #[test]
    fn day09_test_2() {
        assert_eq!(61, find_encryption_weakness(&vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576], 5));
    }
}