

fn part1(input: &Vec<String>) -> String {

    let mut input: Vec<_> = input.iter().map(|x| x.parse::<i64>().unwrap()).collect();

    input.push(0); // socket
    input.sort();
    input.push(input.last().unwrap() + 3); // device

    let mut one_jolt_count = 0;
    let mut three_jolt_count = 0;

    for i in 0..input.len() - 1 {
        let delta = input[i + 1] - input[i];
        match delta {
            1 => one_jolt_count += 1,
            2 => (),
            3 => three_jolt_count += 1,
            _ => panic!()
        }
    }


    let answer = three_jolt_count * one_jolt_count;
    answer.to_string()
}


fn part2(input: &Vec<String>) -> String {
    
    let mut input: Vec<_> = input.iter().map(|x| x.parse::<i64>().unwrap()).collect();

    input.push(0); // socket
    input.sort();
    input.push(input.last().unwrap() + 3); // device


    let mut combination_counts: Vec<usize> = vec![0; input.len()];
    combination_counts[0] = 1;
    
    for i in 0..combination_counts.len(  ) {
        for j in 1..=3 {
            if j <= i {
                if input[i] - input[i - j] <= 3{
                    combination_counts[i] += combination_counts[i - j];
                }
            }
        }
    }
    combination_counts.last().unwrap().to_string()
}

fn main() { 
    lib_aoc::run_with_test("day10", Some(part1), Some(part2));

}
