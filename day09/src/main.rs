

fn find_invalid_number(input: &Vec<i64>) -> i64 {
    let preamble_length = if input.len() > 25 { 25 } else { 5 };

    for i in preamble_length..input.len() {
        let current = input[i];
        let mut current_is_ok = false;
        for a in i-preamble_length..i {
            for b in i-preamble_length..i {
                let a = input[a];
                let b = input[b];

                if a + b == current {
                    current_is_ok = true;
                }
            }
        }

        if !current_is_ok {
            return current;
        }
    }
    
    panic!();
}

fn part1(input: &Vec<String>) -> String {
    let input: Vec<i64> = input.iter().map(|x| x.parse().unwrap()).collect();
    find_invalid_number(&input).to_string()
}


fn part2(input: &Vec<String>) -> String {
    let input: Vec<i64> = input.iter().map(|x| x.parse().unwrap()).collect();
    let key = find_invalid_number(&input);

    let len = input.len();
    let mut dp_table = Vec::<i64>::new();
    //dp_table.reserve(len*(len-1)/2); 
    dp_table.resize_with(len*len, || 0);

    let get = |dp_table: &Vec<_>, col, row| dp_table[col*len + row];
    let set = |dp_table: &mut Vec<_>, col, row, value| { dp_table[col*len + row] = value; };

    
    for row in 0..len {
        set(&mut dp_table, 0, row, input[row]);
    }

    for col in 1..len {
        for row in 0..len-col {
            let a = get(&dp_table, col-1, row);
            let b = get(&dp_table, 0, row+col);

            set(&mut dp_table, col, row, a + b);
        }
    }

    for row in 0..len {
        set(&mut dp_table, 0, row, 0);
    }

    let dp_index = dp_table.iter().position(|&value| value == key).unwrap();

    let dp_row = dp_index % len; 
    let dp_col = dp_index / len;

    let input_index_1 = dp_row;
    let input_index_2 = dp_row + dp_col;

    let min = input[input_index_1..=input_index_2].iter().min().unwrap();
    let max = input[input_index_1..=input_index_2].iter().max().unwrap();
    let answer = min+max;

    return answer.to_string();
}

fn main() { 
    lib_aoc::run_with_test("day09", Some(part1), Some(part2));

}
