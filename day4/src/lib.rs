//Please don't even bother to look at this code, it's a real mess, I hate parsing inputs with Rust

pub struct Passport{
    pub byr: usize, // >=1920 <=2002
    pub iyr: usize, // >=2010 <=2020
    pub eyr: usize, // >=2020 <=2030
    pub hgt: String, // number + cm/inch
    pub hcl: String, // #FFFFFF
    pub ecl: String, // amb blu brn gry grn hzl oth
    pub pid: String, // nine digit number with leading zeroes
    pub cid: bool
}
impl Passport{
    pub fn check_basic_validity(&self)->bool{
        self.byr!=0 && self.iyr!=0 && self.eyr!=0 && self.hgt.len()>0 && self.hcl.len()>0 && self.ecl.len()>0 && self.pid.len()>0
    }
    pub fn print(&self){
        println!("(
            byr:{},
            iyr:{},
            eyr:{},
            hgt:{},
            hcl:{},
            ecl:{},
            pid:{},
            cid:{}
)"
        ,self.byr,self.iyr,self.eyr,self.hgt,self.hcl,self.ecl,self.pid,self.cid)
    }
    pub fn check_validity(&self) -> bool{
        self.byr>=1920 && self.byr<=2002 &&
        self.iyr>=2010 && self.iyr<=2020 &&
        self.eyr>=2020 && self.eyr<=2030 &&
        {
            match self.hgt.ends_with("cm"){
                true => { 
                    let parts:Vec<&str> = self.hgt.split("cm").collect();
                    parts[0].parse::<usize>().unwrap_or(1)>=150 && parts[0].parse::<usize>().unwrap_or(1)<=193
                },
                false => {
                    let parts:Vec<&str> = self.hgt.split("in").collect();
                    parts[0].parse::<usize>().unwrap_or(1)>=59 && parts[0].parse::<usize>().unwrap_or(1)<=76
                },
            }
        } &&
        {
            let parts:Vec<&str> = self.hcl.split("#").collect();
            println!("\n\n{}: {}\n\n",parts.get(1).unwrap_or(&"bla"),parts.get(1).unwrap_or(&"bla").len());
            parts.get(1).unwrap_or(&"bla").len() == 6 && parts.get(1).unwrap_or(&"bla").chars().all(|c| c<='f')
        }&&
        {
            match self.ecl.as_str(){
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false
            }
        }&&
        {
            self.pid.len()==9 && self.pid.chars().all(char::is_numeric)
        }

    }
}


pub fn check_passport_validity(input: &String) -> usize{
    let mut valid_count_passports = 0;
    let mut passport = Passport { byr: 0, iyr: 0, eyr: 0, hgt: String::from(""), hcl: String::from(""), ecl: String::from(""), pid: String::from(""), cid: false };
    for l in input.lines(){
        if l.len()!=0 {
            for couple in l.split(' '){
                let parts:Vec<&str> = couple.split(':').collect();
                match parts[0]{
                    "byr" => passport.byr = parts[1].parse().unwrap(),
                    "iyr" => passport.iyr = parts[1].parse().unwrap(),
                    "eyr" => passport.eyr = parts[1].parse().unwrap(),
                    "hgt" => passport.hgt = String::from(parts[1]),
                    "hcl" => passport.hcl = String::from(parts[1]),
                    "ecl" => passport.ecl = String::from(parts[1]),
                    "pid" => passport.pid = String::from(parts[1]),
                    "cid" => passport.cid = true,
                    _ => println!("Error in passport format")
                }
            }
        }else{
            //println!("Check validity...{}",valid_count_passports);
            //passport.print();
            if passport.check_basic_validity(){
                println!("Valid passport");
                valid_count_passports+=1;
            }
            passport = Passport { byr: 0, iyr: 0, eyr: 0, hgt: String::from(""), hcl: String::from(""), ecl: String::from(""), pid: String::from(""), cid: false };
        }
    }
    valid_count_passports
}

pub fn check_passport_validity_2(input: &String) -> usize{
    let mut valid_count_passports = 0;
    let mut passport = Passport { byr: 0, iyr: 0, eyr: 0, hgt: String::from(""), hcl: String::from(""), ecl: String::from(""), pid: String::from(""), cid: false };
    for l in input.lines(){
        if l.len()!=0 {
            for couple in l.split(' '){
                let parts:Vec<&str> = couple.split(':').collect();
                match parts[0]{
                    "byr" => passport.byr = parts[1].parse().unwrap(),
                    "iyr" => passport.iyr = parts[1].parse().unwrap(),
                    "eyr" => passport.eyr = parts[1].parse().unwrap(),
                    "hgt" => passport.hgt = String::from(parts[1]),
                    "hcl" => passport.hcl = String::from(parts[1]),
                    "ecl" => passport.ecl = String::from(parts[1]),
                    "pid" => passport.pid = String::from(parts[1]),
                    "cid" => passport.cid = true,
                    _ => println!("Error in passport format")
                }
            }
        }else{
            //println!("Check validity...{}",valid_count_passports);
            passport.print();
            if passport.check_validity(){
                println!("Valid passport");
                valid_count_passports+=1;
            }
            passport = Passport { byr: 0, iyr: 0, eyr: 0, hgt: String::from(""), hcl: String::from(""), ecl: String::from(""), pid: String::from(""), cid: false };
        }
    }
    valid_count_passports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1(){
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

";
        assert_eq!(1,check_passport_validity(&String::from(input)));
    }

    #[test]
    fn test_example2(){
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

";
        assert_eq!(2,check_passport_validity(&String::from(input)));
    }

    #[test]
    fn test_example3(){
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719


";
        assert_eq!(4,check_passport_validity_2(&String::from(input)));
    }
    #[test]
    fn test_example4(){
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

"
        
    ;
        assert_eq!(0,check_passport_validity_2(&String::from(input)));
    }

}