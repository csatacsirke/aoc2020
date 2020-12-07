use std::collections::hash_map::HashMap;
use regex::Regex;

struct Passport {
    entries: HashMap<String, String>
}

impl Passport {
    fn new() -> Passport {
        return Passport {
            entries: HashMap::new()
        };
    }
}

fn is_valid_passport(passport: &Passport) -> bool {
    let attributes = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" /*, "cid"*/ ];
    for attribute in attributes.iter() {
        if !passport.entries.contains_key(&attribute.to_string()) {
            return false;
        }
    }
    return true;
}

fn is_byr_valid(value: &String) -> bool {
    // safety net for 4 digits...
    let regex_year = Regex::new(r"^(\d{4})$").unwrap();
    let value = &regex_year.captures(value).unwrap()[1];
    let value = value.parse::<i32>().unwrap();
    return value >= 1920 && value <= 2002;
}

fn is_iyr_valid(value: &String) -> bool {
    // safety net for 4 digits...
    let regex_year = Regex::new(r"^(\d{4})$").unwrap();
    let value = &regex_year.captures(value).unwrap()[1];
    let value = value.parse::<i32>().unwrap();
    return value >= 2010 && value <= 2020;
}

fn is_eyr_valid(value: &String) -> bool {
    // safety net for 4 digits...
    let regex_year = Regex::new(r"^(\d{4})$").unwrap();
    let value = &regex_year.captures(value).unwrap()[1];
    let value = value.parse::<i32>().unwrap();
    return value >= 2020 && value <= 2030;
}

fn is_hgt_valid(value: &String) -> bool {
    let regex_cm = Regex::new(r"^(\d+)cm$").unwrap();
    if let Some(captures) = regex_cm.captures(value) {
        let height_cm = captures[1].parse::<i32>().unwrap();
        return height_cm >= 150 && height_cm <= 193;
    }

    let regex_in = Regex::new(r"^(\d+)in$").unwrap();
    if let Some(captures) = regex_in.captures(value) {
        let height_in = captures[1].parse::<i32>().unwrap();
        return height_in >= 59 && height_in <= 76;
    }

    return false;
}

fn is_hcl_valid(value: &String) -> bool {
    let regex_hair = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    regex_hair.is_match(value)
}

fn is_ecl_valid(value: &String) -> bool {
    let valid_values = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    return valid_values.contains(&value.as_str());
}

fn is_pid_valid(value: &String) -> bool {
    let regex_hair = Regex::new(r"^[0-9]{9}$").unwrap();
    regex_hair.is_match(value)
}

fn is_cid_valid(_: &String) -> bool {
    return true;
}


fn is_valid_passport_with_attributes(passport: &Passport) -> bool {
    let attributes = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];
    
    for attribute in attributes.iter() {

        let value = match passport.entries.get(&attribute.to_string()) {
            Some(value) => value,
            _ => return false,
        };
        

        let is_attribute_valid = match *attribute {
            "byr" => is_byr_valid(value),
            "iyr" => is_iyr_valid(value),
            "eyr" => is_eyr_valid(value),
            "hgt" => is_hgt_valid(value),
            "hcl" => is_hcl_valid(value),
            "ecl" => is_ecl_valid(value),
            "pid" => is_pid_valid(value),
            "cid" => is_cid_valid(value),
            _ => panic!(),
        };

        if !is_attribute_valid {
            //println!("invalid: {}:{}", attribute, value);
            return false;
        }
    }
    return true;
}

fn parse_attributes(line: &String) -> Vec<(String, String)> {
    let entry_regex = Regex::new(r"([^\s]+):([^\s]+)").unwrap();

    let mut attributes = Vec::new();

    for regex_match in entry_regex.captures_iter(line) {
        let key = regex_match[1].to_string();
        let value = regex_match[2].to_string();

        attributes.push((key, value));
    }

    return attributes;
}

fn parse_input(input: &Vec<String>) -> Vec<Passport> {
    let mut passports = Vec::<Passport>::new();
    passports.push(Passport::new());

    for line in input.iter() {
        if line.len() == 0 {
            passports.push(Passport::new());
        } else {
            let attributes = parse_attributes(&line);
            let passport = &mut passports.last_mut().unwrap();
            passport.entries.extend(attributes.into_iter());
        }
    }

    return passports;
}


fn part1(input: &Vec<String>) -> String {
    let passports = parse_input(&input);

    let mut valid_passport_count = 0;

    for passport in passports {
        if is_valid_passport(&passport) {
            valid_passport_count += 1;
        }
    }


    return valid_passport_count.to_string();
}



fn part2(input: &Vec<String>) -> String {
    let passports = parse_input(&input);

    let mut valid_passport_count = 0;

    for passport in passports {
        if is_valid_passport_with_attributes(&passport) {
            valid_passport_count += 1;
        }
    }


    return valid_passport_count.to_string();
}

fn main() { 
    lib_aoc::run_with_test("day04", Some(part1), Some(part2));
}