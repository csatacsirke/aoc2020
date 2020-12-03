// use aoc_2020::read_input

use std::fs;
use std::collections::HashSet;


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