
fn parse_boarding_pass(value: &str) -> i64 {
    let mut seat_id = 0;
    for char in value.chars() {
        seat_id *= 2;
        seat_id += match char {
            'R' => 1,
            'B' => 1,
            'L' => 0,
            'F' => 0,
            _ => panic!()
        };
    }

    return seat_id;

}

fn part1(input: &Vec<String>) -> String {
    

    input.iter()
        .map(|str| parse_boarding_pass(str))
        .max()
        .unwrap()
        .to_string()

}


fn part2(input: &Vec<String>) -> String {
    let mut ids = input.iter()
        .map(|str| parse_boarding_pass(str))
        .collect::<Vec<_>>();

    ids.sort();
    
    for i in 0..ids.len()-1 {
        if ids[i+1] - ids[i] == 2 {
            return (ids[i+1] - 1).to_string();
        }
    }

    panic!();
}

fn main() { 
    lib_aoc::run_with_test("day05", Some(part1),  None);
    
    let input = lib_aoc::read_input("day05/input/input.txt").unwrap();
    println!("part 2 - real : {}", part2(&input));

}