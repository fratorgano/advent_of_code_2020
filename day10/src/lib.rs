use std::collections::BTreeMap;

pub fn adapters_chain_differences(numbers:&mut Vec<usize>) -> usize{
    numbers.push(0);
    numbers.push(*numbers.iter().max().unwrap()+3);
    numbers.sort();
    let mut jolt_diff_3 = 0;
    let mut jolt_diff_1 = 0;
    for i in 1..numbers.len(){
        match numbers[i]-numbers[i-1]{
            3 => {/*  println!("Difference of 3 between: {} {} ({})",numbers[i-1],numbers[i],jolt_diff_3+1);*/jolt_diff_3+=1},
            1 => { /* println!("Difference of 1 between: {} {} ({})",numbers[i-1],numbers[i],jolt_diff_1+1); */jolt_diff_1+=1},
            _ =>()
        }
    }
    jolt_diff_1*jolt_diff_3
}
pub fn adapters_chain_options(numbers:&Vec<usize>) -> usize{
    rec(numbers, 0, &mut BTreeMap::new())
}

fn rec(ads: &Vec<usize>, i: usize, cache: &mut BTreeMap<usize, usize>) -> usize {
    if cache.contains_key(&i) {
        return *cache.get(&i).unwrap();
    }

    if i+1 == ads.len(){
        return 1;
    }
    
    let mut res = 0;
    if i+1 < ads.len() && ads[i+1] - ads[i] <= 3 {
        res += rec(&ads, i+1, cache);
    }
    if i+2 < ads.len() && ads[i+2] - ads[i] <= 3 {
        res += rec(&ads, i+2, cache);
    }
    if i+3 < ads.len() && ads[i+3] - ads[i] <= 3 {
        res += rec(&ads, i+3, cache);
    }
    cache.insert(i, res);
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day10_test() {
        let mut vec = vec![16,10,15,5,1,11,7,19,6,12,4];
        assert_eq!(7*5, adapters_chain_differences(&mut vec));
        vec.sort();
        assert_eq!(8, adapters_chain_options(&vec))
    }

    #[test]
    fn day10_test_2() {
        let mut vec =  vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];
        assert_eq!(220, adapters_chain_differences(&mut vec));
        vec.sort();
        assert_eq!(19208, adapters_chain_options(&vec));
    }
}