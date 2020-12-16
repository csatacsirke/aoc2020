use lib_aoc::*;


fn part1_2_single(input: &str, index_to_find: usize) -> String {
    let input = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();


        
    let mut spoken_numbers = HashMap::<usize, usize>::new();
    spoken_numbers.extend(input.iter().enumerate().map(|(i, &e)| (e, i)));


    let mut current_index = input.len() - 1;
    let mut last_number = *input.iter().last().unwrap();
    
    spoken_numbers.remove(&last_number);

    loop {

        //println!("[{}]: {}", current_index + 1, last_number);
        
        if current_index + 1 == index_to_find {
            return last_number.to_string();
        }


        // calculate next
        let next_number = if let Some(spoken_index) = spoken_numbers.get(&last_number) {
            current_index - spoken_index
        } else {
            0
        };
          

        // update variables
        spoken_numbers.insert(last_number, current_index);
        current_index += 1;
        last_number = next_number;
        
    }
    
}

fn part1(input: &Vec<String>) -> String {
    part1_2_single(&input[0], 2020)
}

fn part2(input: &Vec<String>) -> String {
    part1_2_single(&input[0], 30000000)
}

fn main() { 

    lib_aoc::run_with_test("day15", Some(part1), Some(part2));

    debug_assert!(part1_2_single("0,3,6", 30000000) == "175594");
    

}

