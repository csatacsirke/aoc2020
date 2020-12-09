use regex::Regex;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

#[derive(Clone)]
struct Rule {
    key: String,
    children: HashMap<String, i64>
}

fn parse_rule(input: &String) -> Rule {
    let key_regex = Regex::new(r"^(.*) bags contain").unwrap();
    let key = key_regex.captures(input).unwrap()[1].to_string();

    let mut rule = Rule {
        key: key,
        children: HashMap::new()
    };

    let child_regex = Regex::new(r"([0-9]+) (.*?) bag[s]?[\.,]").unwrap();
    for regex_match in child_regex.captures_iter(&input) {
        let count = regex_match[1].parse::<i64>().unwrap();
        let color = regex_match[2].to_string();
        rule.children.insert(color, count);
    }

    rule
}


fn part1(input: &Vec<String>) -> String {
    let rules : HashMap<_, _> = input.iter()
        .map(|x| (parse_rule(&x).key.clone(), parse_rule(&x).clone())).collect();
    let root = String::from("shiny gold");
    
    let keys : Vec<String> = rules.keys().cloned().collect();

    
    let mut unused: HashSet<String> = keys.iter().cloned().collect();
    let mut used: HashSet<String> = HashSet::new();
    used.insert(root);

    loop {
        let mut new_keys: HashSet<String> = HashSet::new();
        for key in unused.iter() {
            let rule = &rules.get(key).unwrap();
            for (child_name, _count) in rule.children.iter() {
                if used.contains(child_name) {
                    new_keys.insert(key.clone());
                }
            }
        }

        for new_key in new_keys.iter() {
            unused.remove(new_key);
            used.insert(new_key.clone());
        }

        if new_keys.len() == 0 {
            break;
        }
    }

    // -1 mert a shiny gold eleve benne van
    (used.len()-1).to_string()
}


fn part2(input: &Vec<String>) -> String {
    let rules : HashMap<_, _> = input.iter()
        .map(|x| (parse_rule(&x).key.clone(), parse_rule(&x).clone())).collect();
    let root = "shiny gold";
    
    let mut remaining = Vec::<(String, i64)>::new();
    // let iterator = rules.get(&root.to_string()).unwrap().children.iter().map(|(a, b)| (a.clone(), b.clone()));
    // remaining.extend(iterator);
    
    remaining.extend(
        rules.get(&root.to_string())
        .unwrap()
        .children
        .iter()
        .map(|(a, b)| (a.clone(), b.clone()))
    );

    let mut total_count = 0;

    while remaining.len() > 0 {
        let (key, count) = remaining.pop().unwrap();
        total_count += count;

        remaining.extend(
            rules.get(&key)
            .unwrap()
            .children
            .iter()
            .map(|(a, b)| (a.clone(), b.clone() * count))
        );
    }

    total_count.to_string()
}



fn main() { 
    lib_aoc::run_with_test("day07", Some(part1),  Some(part2));

}