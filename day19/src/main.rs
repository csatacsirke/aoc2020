use lib_aoc::*;

#[derive(Clone, Debug)]
enum Rule {
    Node(Vec<Vec<usize>>),
    Leaf(char)
}


#[derive(Debug, Clone)]
struct Context<'t> {
    rules: &'t HashMap::<usize, Rule>,
    applied_rules: Vec<usize>
}

fn is_valid_message(context: &Context, message: &str) -> bool {

    
    // rules match, but message is longer
    if message.len() < context.applied_rules.len() {
        return false;
    }


    for (message_char, &rule_id) in message.chars().zip(context.applied_rules.iter()) {
        let rule = &context.rules[&rule_id];
        match rule {
            Rule::Leaf(rule_char) => {
                if *rule_char != message_char {
                    debug_assert!(*rule_char == 'a' || *rule_char == 'b');
                    debug_assert!(message_char == 'a' || message_char == 'b');
                    return false;
                }
            },
            _ => {
                break;
            }
        }
    }


    for (rule_index, &rule_id) in context.applied_rules.iter().enumerate() {
        let rule = &context.rules[&rule_id];
        match rule {
            Rule::Node(children) => {
                for child_rule in children {
                    let mut new_applied_rules: Vec<usize> = Vec::new();

                    for (i, &applied_rule_index) in context.applied_rules.iter().enumerate() {
                        if i != rule_index {
                            new_applied_rules.push(applied_rule_index);
                        } else {
                            new_applied_rules.extend(child_rule.iter());
                        }
                    }

                    let new_context = Context {
                        rules: context.rules,
                        applied_rules: new_applied_rules,
                    };

                    if is_valid_message(&new_context, message) {
                        return true;
                    }
                }

                return false;
            },
            _ => {}
        }
    }

    // rules match, but message is longer
    if message.len() != context.applied_rules.len() {
        return false;
    }

    //println!("{}: {:?}", message, context.applied_rules);

    return true;
}

enum Part {
    Part1, Part2
}

fn monster_messages(input: &Vec<String>, part: Part) -> String {
    
    let mut it = input.iter();

    let re_indices = Regex::new(r"(\d+): (.*)").unwrap();
    let re_number = Regex::new(r"(\d+)").unwrap();
    let re_leaf_rule = Regex::new(r#""(\w)""#).unwrap(); // "a"

    let collect_numbers = |str: &str| -> Vec<usize> {
        re_number.find_iter(str)
            .map(|re_match: Match| re_match.as_str().parse().unwrap())
            .collect()
    };

    let mut rules = HashMap::<usize, Rule>::new();

    for line in &mut it {
        if line.is_empty() {
            break;
        }

        let captures = re_indices.captures(line).unwrap();
        let rule_id: usize = captures[1].parse().unwrap();
        let rule_def = captures[2].to_string();
        
        if let Some(captures) = re_leaf_rule.captures(&rule_def) {
            let leaf_rule = &captures[1];
            debug_assert!(leaf_rule.len() == 1);
            let char = leaf_rule.chars().next().unwrap();
            rules.insert(rule_id, Rule::Leaf(char));
        } else {

            let sub_rules: Vec<Vec<usize>> = rule_def.split("|")
                .map(|sub_rule| collect_numbers(sub_rule))
                .collect();


            debug_assert!((1..=2).contains(&sub_rules.len()));
            for rule in &sub_rules {
                if rule_id != 0 {
                    debug_assert!((1..=2).contains(&rule.len()));
                }
            }

            rules.insert(rule_id, Rule::Node(sub_rules));
        }
    }

    if let Part::Part2 = part {
        rules.insert(8, Rule::Node(vec![vec![42], vec![42, 8]]));
        rules.insert(11, Rule::Node(vec![vec![42, 31], vec![42, 11, 31]]));
    }

    let mut valid_message_count = 0;
    for message in &mut it {
        let context = Context {
            rules: &rules,
            applied_rules: vec![0]
        };

        if is_valid_message(&context, &message) {
            valid_message_count += 1
        }

    }

    valid_message_count.to_string()
}

fn part1(input: &Vec<String>) -> String {
    monster_messages(input, Part::Part1)
}


fn part2(input: &Vec<String>) -> String {
    monster_messages(input, Part::Part2)
}

fn main() { 

    lib_aoc::run_with_test("day19", Some(part1), Some(part2));

}

