use regex::Regex;

struct Policy {
    letter: char,
    min: usize,
    max: usize
}

struct Entry {
    policy: Policy,
    password: String
}

impl Entry {
    pub fn is_valid(&self) -> bool {
        let mut char_count : usize = 0;
        for char in self.password.chars() {
            if char == self.policy.letter {
                char_count += 1;
            }
        }

        // tanuljak már valami újat is...
        //return (self.policy.min ..= self.policy.max).contains(&char_count);
        return char_count <= self.policy.max && char_count >= self.policy.min;
    }
}

fn parse_line(line: &String) -> Entry {
    // 17-20 x: zsxjrxkgxxxxxxxmxgxf
    let entry_regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let captures = entry_regex.captures(line).unwrap();
    
    Entry {
        policy: Policy {
            letter: captures[3].chars().next().unwrap(),
            min: captures[1].parse().unwrap(),
            max: captures[2].parse().unwrap()
        },
        password: captures[4].to_string()
    }
    
}

fn part1(input: &Vec<String>) -> String {
    let mut valid_password_count = 0;

    for line in input {
        let entry = parse_line(line);
        if entry.is_valid() {
            valid_password_count += 1;
        }
    }

    return valid_password_count.to_string();
}

fn _part2(_input: &Vec<String>) -> String {
    panic!();
}

fn main() {
    aoc2020::run_with_test("day02", Some(part1), None);
}