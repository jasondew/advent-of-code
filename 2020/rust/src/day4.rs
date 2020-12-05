#![warn(clippy::pedantic)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct Passport(HashMap<String, String>);

impl Passport {
    pub fn load(input: String) -> Vec<Passport> {
        input
            .split("\n\n")
            .map(|lines| lines.parse().unwrap())
            .collect()
    }

    fn required_fields_present(&self) -> bool {
        self.0.get("byr").is_some()
            && self.0.get("iyr").is_some()
            && self.0.get("eyr").is_some()
            && self.0.get("hgt").is_some()
            && self.0.get("hcl").is_some()
            && self.0.get("ecl").is_some()
            && self.0.get("pid").is_some()
    }

    fn is_valid(&self) -> bool {
        self.required_fields_present()
            && between(self.0.get("byr").unwrap(), 1920, 2002)
            && between(self.0.get("iyr").unwrap(), 2010, 2020)
            && between(self.0.get("eyr").unwrap(), 2020, 2030)
            && valid_height(self.0.get("hgt").unwrap())
            && valid_hair_color(self.0.get("hcl").unwrap())
            && valid_eye_color(self.0.get("ecl").unwrap())
            && valid_passport_id(self.0.get("pid").unwrap())
    }
}

fn between(string: &String, from: i32, to: i32) -> bool {
    let value: i32 = string.parse().unwrap();
    from <= value && value <= to
}

fn valid_height(string: &String) -> bool {
    if let Some(rest) = string.strip_suffix("cm") {
        between(&rest.to_owned(), 150, 193)
    } else {
        if let Some(rest) = string.strip_suffix("in") {
            between(&rest.to_owned(), 59, 76)
        } else {
            false
        }
    }
}

fn valid_hair_color(string: &String) -> bool {
    if let Some(rest) = string.strip_prefix("#") {
        i32::from_str_radix(rest, 16).is_ok()
    } else {
        false
    }
}

fn valid_eye_color(string: &String) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&string.as_str())
}

fn valid_passport_id(string: &String) -> bool {
    string.len() == 9 && string.parse::<i32>().is_ok()
}

impl std::str::FromStr for Passport {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, String> = HashMap::new();

        for group in string.trim().split(|c| c == ' ' || c == '\n') {
            let pair: Vec<&str> = group.split(':').collect();
            map.insert(
                (*pair.get(0).unwrap()).into(),
                (*pair.get(1).unwrap()).into(),
            );
        }
        Ok(Passport(map))
    }
}

pub fn part1(passports: &Vec<Passport>) -> usize {
    passports
        .iter()
        .filter(|passport| passport.required_fields_present())
        .count()
}

pub fn part2(passports: &Vec<Passport>) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let passports: Vec<Passport> = Passport::load("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n".into());
        assert_eq!(part1(&passports), 2)
    }

    #[test]
    fn part2_example() {
        let passports: Vec<Passport> = Passport::load("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007\n\npid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 pid:093154719".into());
        assert_eq!(part2(&passports), 3)
    }
}
