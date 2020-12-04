use std::fs;
use std::collections::{HashMap};

#[macro_use] extern crate maplit;


struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}


impl Passport {
    fn from_string(input: &str) -> Option<Passport> {
        let elements: HashMap<&str, &str> = input
            .split_ascii_whitespace()
            .map(|element| element.split(":"))
            .map(|element| {
                let v: Vec<&str> = element.collect();
                (v[0], v[1])
            })
            .collect();
        if elements.contains_key("byr") &&
            elements.contains_key("iyr") &&
            elements.contains_key("eyr") &&
            elements.contains_key("hgt") &&
            elements.contains_key("hcl") &&
            elements.contains_key("ecl") &&
            elements.contains_key("pid") {

            let mut passport = Passport {
                byr: elements["byr"].to_string(),
                iyr: elements["iyr"].to_string(),
                eyr: elements["eyr"].to_string(),
                hgt: elements["hgt"].to_string(),
                hcl: elements["hcl"].to_string(),
                ecl: elements["ecl"].to_string(),
                pid: elements["pid"].to_string(),
                cid: None
            };
            if elements.contains_key("cid") {
                passport.cid = Some(elements["cid"].to_string());
            };
            Some(passport)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        fn is_decimal_digit(c: &char) -> bool {
            c >= &'0' && c <= &'9'
        }

        fn is_hex_digit(c: &char) -> bool {
            is_decimal_digit(c) || (c >= &'a' && c <= &'f') || (c >= &'A' && c <= &'F')
        }

        fn is_valid_byr(byr: &str) -> bool {
            byr.len() == 4 && byr >= "1920" && byr <= "2002"
        };

        fn is_valid_iyr(iyr: &str) -> bool {
            iyr.len() == 4 && iyr >= "2010" && iyr <= "2020"
        }

        fn is_valid_eyr(eyr: &str) -> bool {
            eyr.len() == 4 && eyr >= "2020" && eyr <= "2030"
        }

        fn is_valid_hgt(hgt: &str) -> bool {
            if hgt.len() == 4 && &hgt[2..] == "in" {
                let hgt_value = hgt[..2].parse::<i32>().unwrap();
                hgt_value >= 59 && hgt_value <= 76
            } else if hgt.len() == 5 && &hgt[3..] == "cm" {
                let hgt_value = hgt[..3].parse::<i32>().unwrap();
                hgt_value >= 150 && hgt_value <= 193
            } else {
                false
            }
        }

        fn is_valid_hcl(hcl: &str) -> bool {
            hcl.len() == 7 && hcl.starts_with("#") && hcl[1..].chars().filter(is_hex_digit).count() == 6
        }

        fn is_valid_ecl(ecl: &str) -> bool {
            let colors = hashset!{ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" };
            colors.contains(ecl)
        }

        fn is_valid_pid(pid: &str) -> bool {
            pid.len() == 9 && pid.chars().filter(is_decimal_digit).count() == 9
        }

        return is_valid_byr(&self.byr) &&
            is_valid_iyr(&self.iyr) &&
            is_valid_eyr(&self.eyr) &&
            is_valid_hgt(&self.hgt) &&
            is_valid_hcl(&self.hcl) &&
            is_valid_ecl(&self.ecl) &&
            is_valid_pid(&self.pid)
        ;
    }
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to read input.txt");

    let passports: Vec<Passport> = contents
        .split("\n\n")
        .map(|input| Passport::from_string(input))
        .filter(|passport| passport.is_some())
        .map(|passport| passport.unwrap())
        .collect();

    println!("{}", passports.len());

    let valid_passports = passports.into_iter().filter(|passport| passport.is_valid()).count();
    println!("{}", valid_passports);
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_load_passport_records() {
        let passports: Vec<Passport> = INPUT
            .split("\n\n")
            .map(|input| Passport::from_string(input))
            .filter(|passport| passport.is_some())
            .map(|passport| passport.unwrap())
            .collect();

        assert_eq!(passports.len(), 2);
    }
}