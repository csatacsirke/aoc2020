use lib_aoc::*;


fn part1(input: &Vec<String>) -> String {
    let earliest: i64 = input[0].parse().unwrap();

    let (id, _depature, wait_time) = input[1]
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .map(|id| (id, earliest + (id - (earliest % id)), id - (earliest % id)))
        .min_by_key(|&(_id, depart_time, _wait_time)| depart_time)
        .unwrap();


    let answer = wait_time * id;

    answer.to_string()
}

fn solve_modulo_eq((a1, n1): (i128, i128), (a2, n2): (i128, i128)) -> (i128, i128) {
    // x = a1 mod n1
    // x = a2 mod n2
    // --------------
    
    // assert: n1 n2 are primes

    // step_by cant handle i128
    //for x in (a1 as i128..n1 as i128 * n2 as i128).step_by(n1 as usize) 

    if n1 < n2 {
        return solve_modulo_eq((a2, n2), (a1, n1));
    }

    // let a1 = a1 % n1;
    // let a2 = a2 % n2;

    println!("\t {} {} {} {}", a1, n1, a2, n2);

    let mut x: i128 = a1;
    loop {
    //for x in (a1..n1 * n2).step_by(n1) {
        if x % n2 == a2 % n2  {

            //println!("\t {} {} {} {} -> {}", a1, n1, a2, n2, x);
            return (x, n1*n2);
        }
        x += n1;

        if x > n1*n2 {
            panic!();
        }
    }

}

fn part2_single_line(input: &str) -> String {
    // input consist of primes only ^^
    let (result, _modulo) = input
        .split(',')
        .enumerate()
        .filter_map(|(index, x)| x.parse::<i128>().and_then(|id| Ok((index as i128, id))).ok())
        .map(|(index, id)| (id - (index % id), id)) // wait time to modulo conversion
        .reduce(solve_modulo_eq)
        .unwrap();

    result.to_string()
}

fn part2(input: &Vec<String>) -> String {
    part2_single_line(&input[1])
}

fn test_part2(input: &str, expected: &str) {
    let computed = part2_single_line(input);
    if *computed != *expected {
        println!("'{}' {} should be {}", input, &computed, {expected});
        panic!();
    } else {
        println!("Part2 additional test: '{}'  ok", input);
    }
    
}

fn main() { 
    
    test_part2("7,13,x,x,59,x,31,19", "1068781");
    test_part2("17,x,13,19", "3417");
    test_part2("67,7,59,61", "754018");
    test_part2("67,x,7,59,61", "779210");
    test_part2("67,7,x,59,61", "1261476");
    test_part2("1789,37,47,1889", "1202161486");

    lib_aoc::run_with_test("day13", Some(part1), Some(part2));

}

