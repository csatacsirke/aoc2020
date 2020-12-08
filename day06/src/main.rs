use std::collections::hash_set::HashSet;


fn part1(input: &Vec<String>) -> String {

    let mut set = HashSet::<String>::new();
    let mut sum = 0;
    for line in input.iter() {
        if line.len() == 0 {
            sum += set.len();
            set.clear();
            continue;
        }

        for char in line.chars() {
            set.insert(char.to_string());
        }
    }   

    
    sum += set.len();

    sum.to_string()
}


fn part2(input: &Vec<String>) -> String {
    
    let mut set = HashSet::<String>::new();
    let mut sum = 0;
    let mut first_person = true;
    for line in input.iter() {

        if line.len() == 0 {
            sum += set.len();
            set.clear();
            first_person = true;
            continue;
        }

        if first_person {
            for char in line.chars() {
                set.insert(char.to_string());
            }
            first_person = false;
        } else {
            let mut set2 = HashSet::<String>::new();
            for char in line.chars() {
                set2.insert(char.to_string());
            }
            set = set.intersection(&set2).map(|x| x.to_string()).collect();
        }
        
    }   

    
    sum += set.len();
    
    sum.to_string()
}

fn main() { 
    lib_aoc::run_with_test("day06", Some(part1),  Some(part2));

}