pub fn parse_input_to_keys(input:&String) -> (usize,usize){
    let mut lines = input.lines();
    (lines.next().unwrap().parse().unwrap(),lines.next().unwrap().parse().unwrap())
}

pub fn find_encryption_key(input:&String) -> usize{
    let public_keys = parse_input_to_keys(input);
    let subject_number = 7;
    //find loop size client
    let ls_client = find_loop_size(public_keys.0, subject_number);
    //find loop size server
    //let ls_server = find_loop_size(public_keys.1, subject_number);
    let priv_key = calculate_encryption_key(public_keys.1, ls_client);
    priv_key
}

fn find_loop_size(key:usize, subject_number:usize) -> usize{
    let mut calculated_key = 1;
    let mut loop_size = 0;
    while calculated_key!=key{
        calculated_key *= subject_number;
        calculated_key %= 20201227;
        loop_size += 1;
    }
    loop_size
}

fn calculate_encryption_key(subject_number:usize, loop_size:usize) -> usize{
    let mut encryption_key = 1;
    for _ in 0..loop_size{
        encryption_key *= subject_number;
        encryption_key %= 20201227;
    }
    encryption_key
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_test() {
        let input = String::from("5764801
17807724");
        assert_eq!(14897079, find_encryption_key(&input));
    }

    #[test]
    fn day25_test_2() {
    }
}