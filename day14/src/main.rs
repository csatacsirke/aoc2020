use lib_aoc::*;
use lib_aoc::Reduce;

type MemoryCell = Vec<char>;

fn as_memory_cell(value: i64) -> MemoryCell {
    let mut memory_value: MemoryCell = "000000000000000000000000000000000000".chars().collect();

    for i in 0..memory_value.len() {
        memory_value[i] = if (value >> (35 - i)) & 0x1 != 0 { '1' } else {'0'};
    }

    return memory_value;
}

fn read_value(cell: &MemoryCell) -> i64 {
    let mut value = 0;
    for (i, char) in cell.iter().enumerate() {
        let bit = match char {
            '0' => 0,
            '1' => 1,
            _ => panic!(),
        };
        value += bit << (35 - i);
    }

    return value;
}

fn calc_value(new_value: &MemoryCell, mask: &MemoryCell) -> MemoryCell {
    let mut result = MemoryCell::new();

    for (i, char) in mask.iter().enumerate() {
        let new_char = match char {
            'X' => new_value[i],
            '0' => '0',
            '1' => '1',
            _ => panic!(),
        };

        result.push(new_char);
        
    }

    return result;
}


fn part1(input: &Vec<String>) -> String {
    let _def_memory_value: MemoryCell = "000000000000000000000000000000000000".chars().collect();

    let regex_mask = regex::Regex::new(r"mask = ([X01]+)").unwrap();
    let regex_memset = regex::Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let mut current_mask = MemoryCell::new();
    let mut memory = HashMap::<usize, MemoryCell>::new();

    for line in input {
        if let Some(captures) = regex_mask.captures(line) {
            current_mask = captures[1].to_string().chars().collect();
        } else if let Some(captures) = regex_memset.captures(line) {
            let mem_address: usize = captures[1].parse().unwrap();
            let value = as_memory_cell(captures[2].to_string().parse().unwrap());

            //let value = memory.get(&mem_address).unwrap_or(&def_memory_value).clone();
            let value = calc_value(&value, &current_mask);
            memory.insert(mem_address, value.clone());

            //println!("{}", value.iter().collect::<String>());
        } else {
            panic!();
        }
    }

    let answer = memory.values().map(|value| read_value(value)).reduce(|a, b| a + b).unwrap();
    return answer.to_string();
}


fn calc_memory_mask(old_value: &MemoryCell, mask: &MemoryCell) -> MemoryCell {
    let mut result = old_value.clone();

    for (i, char) in mask.iter().enumerate() {
        let new_char = match char {
            'X' => 'X',
            '0' => old_value[i],
            '1' => '1',
            _ => panic!(),
        };

        result[i] = new_char;
        
    }

    return result;
}

fn for_each_combination<F: FnMut(usize)>(cell: &mut MemoryCell, index: usize, f: &mut F) {
    if index == 36 {
        f(read_value(cell) as usize);
        return;
    }

    if cell[index] == 'X' {

        cell[index] = '0';
        for_each_combination(cell, index + 1, f);

        cell[index] = '1';
        for_each_combination(cell, index + 1, f);

        cell[index] = 'X';
    } else {
        for_each_combination(cell, index + 1, f);
    }

}

fn part2(input: &Vec<String>) -> String {

    let regex_mask = regex::Regex::new(r"mask = ([X01]+)").unwrap();
    let regex_memset = regex::Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let mut current_mask = MemoryCell::new();
    let mut memory = HashMap::<usize, i64>::new();

    for line in input {
        if let Some(captures) = regex_mask.captures(line) {
            current_mask = captures[1].to_string().chars().collect();
        } else if let Some(captures) = regex_memset.captures(line) {

            let mem_address_mask = as_memory_cell(captures[1].parse().unwrap());
            let value = captures[2].to_string().parse().unwrap();

            let mut memory_mask = calc_memory_mask(&mem_address_mask, &current_mask);
            
            println!("memory mask: {}", memory_mask.iter().collect::<String>());
            for_each_combination(&mut memory_mask, 0, &mut |mem_address| {
                println!("\t write to: {}  ({})", mem_address, value);
                memory.insert(mem_address, value);
            })

            

            //println!("{}", value.iter().collect::<String>());
        } else {
            panic!();
        }
    }

    let mut answer = 0;
    for value in memory.values() {
        answer += value;
    }
    return answer.to_string();
}

fn main() { 
    
    lib_aoc::run_with_test("day14", Some(part1), Some(part2));

}

