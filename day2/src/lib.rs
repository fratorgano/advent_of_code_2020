/* use lazy_static::lazy_static;
use regex::Regex; */

pub struct Password{
    password: String,
    range: (usize,usize),
    letter: char
}

impl Password{
    pub fn new(line:&str) -> Password{
        // line format = rangeStart-rangeEnd char: string
        // e.g.:         1-3 a: abcde
        /* lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
        }
        let captures = RE.captures(line).unwrap();
        Password {
            range: (captures.get(1).unwrap().as_str().parse().unwrap(), captures.get(2).unwrap().as_str().parse().unwrap()),
            letter: captures.get(3).unwrap().as_str().chars().next().unwrap(),
            password: captures.get(4).unwrap().as_str().to_string(),
        } */
        let parts: Vec<&str> = line.split_whitespace().collect();

        let times: Vec<usize> = parts[0]
            .split('-')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();

        Password {
            range: (times[0], times[1]),
            letter: parts[1].as_bytes()[0] as char,
            password: String::from(parts[2]),
        }
    }
    /* fn get_password(&self) -> &String {
        &self.password
    }
    fn get_range(&self) -> (usize,usize) {
        self.range
    }
    fn get_letter(&self) -> char {
        self.letter
    } */
    fn validity(&self) -> bool{
        //let mut n = 0;
        /* for c in self.password.chars(){
            if c==self.letter{
                n+=1
            }
        } */
        let n = self.password.chars().fold(0, |acc,c| if c==self.letter {acc+1} else {acc});
        n>=self.range.0 && n<=self.range.1
    }
    fn correct_validity(&self) -> bool{
        let chars:Vec<char> = self.password.chars().collect();
        (chars[self.range.0-1]==self.letter) ^ (chars[self.range.1-1]==self.letter)
    }
}

pub fn check_all_pass_val(s: &String) -> i32{
    /* let mut n_valid = 0;
    for line in s.lines(){
        let p = Password::new(line);
        if p.validity(){
            n_valid+=1;
        }
    n_valid
    } */
    s.lines().fold(0, |acc, s|if Password::new(s).validity() {acc+1} else {acc})
}
pub fn check_all_pass_correct_val(s: &String) -> i32{
    /* let mut n_valid = 0;
    for line in s.lines(){
        let p = Password::new(line);
        if p.correct_validity(){
            n_valid+=1;
        }
    n_valid
    } */
    s.lines().fold(0, |acc, s|
        if Password::new(s).correct_validity() {acc+1} else {acc}
    )
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_password1(){
        let p = Password::new(&String::from("1-3 a: abcde"));
        /* assert_eq!("abcde", p.get_password());
        assert_eq!('a', p.get_letter());
        assert_eq!((1,3), p.get_range()); */
        assert!(p.validity())
    }
    #[test]
    fn test_new_password2(){
        let p = Password::new(&String::from("1-3 b: cdefg"));
        /* assert_eq!("cdefg", p.get_password());
        assert_eq!('b', p.get_letter());
        assert_eq!((1,3), p.get_range()); */
        assert!(!p.validity())
    }
    #[test]
    fn test_new_password3(){
        let p = Password::new(&String::from("2-9 c: ccccccccc"));
        /* assert_eq!("ccccccccc", p.get_password());
        assert_eq!('c', p.get_letter());
        assert_eq!((2,9), p.get_range()); */
        assert!(p.validity())
    }
}
