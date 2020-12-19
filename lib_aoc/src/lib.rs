use std::path::Path;
use std::fs;
use thiserror::Error;
pub use coord_2d::*;
pub use reduce::*;
pub use regex::*;
pub use std::collections::hash_map::HashMap;
pub use std::collections::hash_set::HashSet;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
   
    #[error(transparent)]
    String(#[from] std::string::FromUtf8Error),
}

pub fn read_input<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, Error> {
    let file_contents = fs::read(file_path)?;
    let file_contents = String::from_utf8(file_contents)?;
    let lines : Vec<String> = file_contents.lines().map(|line| line.to_string()).collect();
    

    Ok(lines)
}


pub type AocProgram = fn(&Vec<String>) -> String;

pub fn run_test(program: AocProgram, input: &Vec<String>, expected_output: &str) {
    let output = program(input);
    if output != expected_output {
        println!("Not equal: '{}' '{}'", expected_output, output);
    }
    assert_eq!(output, expected_output);
}

//pub fn run_test_from_file(program: AocProgram, input: &Vec<String>, expected_output: &str) {
//    // todo
//}

pub fn run_with_test(day: &str, part1: Option<AocProgram>, part2: Option<AocProgram>) {
    let _cwd = std::env::current_dir();

    println!("Running '{}'", day);
    
    let example_input = read_input(format!("{day}/input/example.txt", day=day)).unwrap();
    let real_input = read_input(format!("{day}/input/input.txt", day=day)).unwrap();

    if let Some(part1) = part1 {
        let example_output = fs::read(format!("{day}/input/example_answer.txt", day=day));
        let example_output = String::from_utf8(example_output.unwrap()).unwrap().trim().to_string();

        run_test(part1, &example_input, &example_output);

        println!("part 1 test OK");

        let part1_answer = part1(&real_input);

        println!("part 1 answer: {}", part1_answer);
    }


    if let Some(part2) = part2 {
        // nem kötelező, hogy létezzen
        let example2_input = read_input(format!("{day}/input/example2.txt", day=day));
        let example2_input = match example2_input {
            Ok(input) => input,
            _ => example_input
        };

        let example_output = fs::read(format!("{day}/input/example_answer2.txt", day=day));
        let example_output = String::from_utf8(example_output.unwrap()).unwrap().trim().to_string();
        run_test(part2, &example2_input, &example_output);

        println!("part 2 test OK");

        let part2_answer = part2(&real_input);
        println!("part 2 answer: {}", part2_answer);

    }

    println!("finished.");
}
