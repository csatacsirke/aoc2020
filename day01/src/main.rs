use std::path::Path;
use std::fs;
use std::collections::HashSet;
use regex::Regex;

type AocProgram = fn(&Vec<String>) -> String;

fn part1(input: &Vec<String>) -> String {
    
    let input : Vec<_> = input.into_iter()
        .map(|string| string.parse::<i64>().unwrap())
        .collect();

    let numbers : HashSet<i64> = input.iter()
        .map(|x| *x)
        .collect();

    let element = input.iter()
        .find(|x| numbers.contains(&(2020-*x)))
        .unwrap();

    let answer = element * (2020-element);

    return answer.to_string();
}

fn part2(input: &Vec<String>) -> String {

    let input : Vec<_> = input.into_iter()
        .map(|string| string.parse::<i64>().unwrap())
        .collect();

    let numbers : HashSet<i64> = input.iter()
        .map(|x| *x)
        .collect();

    for a in input.iter() {
        let b = input.iter()
            .find(|b| numbers.contains(&(2020 - a - *b)));
        
        if let Some(b) = b {
            return (a * b * (2020 - a - b)).to_string();
        }
    }

    panic!();
}


pub fn read_input<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, aoc2020::Error> {
    let file_contents = fs::read(file_path)?;
    let file_contents = String::from_utf8(file_contents)?;

    //let input_elements = file_contents.split(",");
    let regex = Regex::new(r"\w+").unwrap();
    let strings = regex.find_iter(&file_contents)
        .map(|m| String::from(m.as_str()))
        .collect::<Vec<String>>();
    //let matches = regex.split(&file_contents).into_iter().collect();
    //let input_elements = file_contents.split()
    //let strings = input_elements.map(String::from).collect::<Vec<String>>();
    
    Ok(strings)
}


fn run_test(program: AocProgram, input: &Vec<String>, expected_output: &str) {
    let output = program(input);
    assert_eq!(output, expected_output);
}

fn main() -> Result<(), aoc2020::Error> {

    println!("Start\r\n");


    let input : Vec<_> = aoc2020::read_input(r"f:\Programming\AoC2020\inputs\01\example_p1_1.txt")?;
        
    let output = fs::read(r"f:\Programming\AoC2020\inputs\01\example_p1_1.answer.txt");
    let output = String::from_utf8(output.unwrap()).unwrap().trim().to_string();
    

    run_test(part1, &input, &output);
    run_test(part2, &input, "241861950");
    
    let input : Vec<_> = aoc2020::read_input(r"f:\Programming\AoC2020\inputs\01\input.txt")?;
    
    let answer = part1(&input);
    println!("part1 {}", answer);
    
    let answer = part2(&input);
    println!("part2 {}", answer);

    return Ok(());
}