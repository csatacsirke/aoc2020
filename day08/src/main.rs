use regex::Regex;
//use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

struct Processor<'a> {
    accumulator: i64,
    instruction_pointer: i64,
    input: &'a Vec<String>,
}

impl<'a> Processor<'a> {
    fn new(input: &'a Vec<String>) -> Self {
        Processor {
            accumulator: 0,
            instruction_pointer: 0,
            input: input,
        }
    }

    fn did_terminate(&self) -> bool {
        let ip = self.instruction_pointer as usize;
        return ip == self.input.len();
    }

    fn run_next_command(&mut self) {
        let ip = self.instruction_pointer as usize;
        let instruction = &self.input[ip];
        self.run_command(&instruction);
    }


    fn run_command(&mut self, command: &str) {
        let regex_command = Regex::new(r"(\w+) (.*)").unwrap();
        let captures = regex_command.captures(command).unwrap();
        let instruction = captures[1].to_string();
        let args = captures[2].to_string();
        self.run_command_with_args(&instruction, &args);
    }

    fn run_command_with_args(&mut self, command: &str, args: &str) {
        match command {
            "acc" => self.acc(args),
            "nop" => self.nop(args),
            "jmp" => self.jmp(args),
            _ => panic!()
        };
    }

    fn nop(&mut self, _args: &str) {
        self.instruction_pointer += 1;
    }

    fn acc(&mut self, args: &str) {
        let delta: i64 = args.parse().unwrap();
        self.accumulator += delta;
        self.instruction_pointer += 1;
    }

    fn jmp(&mut self, args: &str) {
        let delta: i64 = args.parse().unwrap();
        self.instruction_pointer += delta;
    }
}

fn part1(input: &Vec<String>) -> String {

    let mut processor = Processor::new(input);
    let mut instructions = HashSet::<i64>::new();

    loop {
        processor.run_next_command();

        if instructions.contains(&processor.instruction_pointer) {
            return processor.accumulator.to_string();
        }
        //println!("ip: {}", processor.instruction_pointer);
        instructions.insert(processor.instruction_pointer);
    }
    

    //panic!();
}

enum TestResult {
    InfiniteLoop,
    ValidProgramResult(i64)
}

fn test_program(input: &Vec<String>) -> TestResult {

    let mut processor = Processor::new(input);
    let mut instructions = HashSet::<i64>::new();

    loop {
        processor.run_next_command();

        if instructions.contains(&processor.instruction_pointer) {
            return TestResult::InfiniteLoop;
        }

        if processor.did_terminate() {
            return TestResult::ValidProgramResult(processor.accumulator);
        }
        
        instructions.insert(processor.instruction_pointer);
    }
    

}

fn part2(input: &Vec<String>) -> String {
    for i in 0..input.len() {
        let mut corrected_input = input.clone();
        
        match &input[i][0..3] {
            "nop" => corrected_input[i] = corrected_input[i].replace("nop", "jmp"),
            "jmp" => corrected_input[i] = corrected_input[i].replace("jmp", "nop"),
            _ => ()
        }
        
        if let TestResult::ValidProgramResult(result) = test_program(&corrected_input) {
            return result.to_string();
        }
    }

    panic!();
}

fn main() { 
    lib_aoc::run_with_test("day08", Some(part1),  Some(part2));

}