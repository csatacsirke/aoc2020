use core::ops::Range;
use lib_aoc::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Token {
    GroupStart,
    GroupEnd,
    Number(i64),
    OpMul,
    OpAdd,
    Whitespace,
}

impl From<&str> for Token {
    fn from(str: &str) -> Self { 
        match str {
            "(" => Token::GroupStart,
            ")" => Token::GroupEnd,
            "*" => Token::OpMul,
            "+" => Token::OpAdd,
            " " => Token::Whitespace,
            _ => Token::Number(str.parse::<i64>().unwrap()),
        }
    }
}

fn try_reduce(stack: &mut Vec<Token>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let mut did_reduce = false;

    if let Token::Number(number2) = stack[stack.len()-1] {
        if stack.len() >= 3 {
            let op = stack[stack.len()-2];
            
            if let Token::OpAdd = op {

                let number1 = match stack[stack.len()-3] {
                    Token::Number(n) => n,
                    _ => panic!(),
                };
            
                let result = number1 + number2;

                
                //println!("{} + {} = {}", number1, number2, result);
                
                stack.pop();
                stack.pop();
                stack.pop();

                stack.push(Token::Number(result));
                did_reduce = true;
            }

            if let Token::OpMul = op {

                let number1 = match stack[stack.len()-3] {
                    Token::Number(n) => n,
                    _ => panic!(),
                };

                let result = number1 * number2;

                //println!("{} * {} = {}", number1, number2, result);
                
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(Token::Number(result));
                did_reduce = true;
            }
        }

    }

    if let Token::GroupEnd = stack[stack.len()-1] {
        debug_assert!(stack.len() > 2);

        stack.pop();

        let number = match stack.pop().unwrap() {
            Token::Number(number) => number,
            _ => panic!(),
        };

        let _group_start = stack.pop().unwrap();
        match _group_start {
            Token::GroupStart => (),
            _ => panic!(),
        }

        stack.push(Token::Number(number));

        did_reduce = true;
    }

    did_reduce
}

fn parse_expression(line: &str) -> i64 {

    let re = regex::Regex::new(r"\(|\)|\s+|\w+|\+|\*").unwrap();

    let mut it = re.find_iter(line);

    
    let mut stack = Vec::<Token>::new();


    while let Some(token_as_str) = it.next().and_then(|token| Some(token.as_str())) {

        let token: Token = token_as_str.into();
        
        
        if let Token::Whitespace = token {
            continue;
        }

        //println!("'{}'", token_as_str);

        stack.push(token);

        while try_reduce(&mut stack) { }


    }

    debug_assert!(stack.len() == 1);
    
    let number = match stack.pop().unwrap() {
        Token::Number(number) => number,
        _ => panic!(),
    };

    number
}


fn part1(input: &Vec<String>) -> String {

    let answer = input.iter()
        .map(|x| parse_expression(x))
        .reduce(|a, b| a + b)
        .unwrap();

    return answer.to_string();
}



// enum Expression {
//     Group(Box<Expression>),
//     Add(Box<Expression>, Box<Expression>),
//     Mul(Box<Expression>, Box<Expression>),
//     Token(Token),
// }

fn try_single_reduce_single_expression(slice: &[Token]) -> Option<(Token, Range<usize>)> {
    if slice.len() < 3 {
        return None;
    }

    for i in 0..slice.len()-2 {
        let tokens = &slice[i..i+3];
        match tokens[0] {
            Token::GroupStart => {},
            _ => {continue;}
        }

        let number_token = match tokens[1] {
            Token::Number(_) => tokens[1], 
            _ => {continue;}
        };

        match tokens[2] {
            Token::GroupEnd => {}
            _ => {continue;}
        }

        return Some((number_token, i..i+3));
    }

    for i in 0..slice.len()-2 {
        let tokens = &slice[i..i+3];
        let number1 = match tokens[0] {
            Token::Number(number) => number,
            _ => {continue;}
        };

        match tokens[1] {
            Token::OpAdd => tokens[1], 
            _ => {continue;}
        };

        let number2 = match tokens[2] {
            Token::Number(number) => number, 
            _ => {continue;}
        };

        return Some((Token::Number(number1 + number2), i..i+3));
    }

    
    for i in 0..slice.len()-2 {
        let tokens = &slice[i..i+3];
        let number1 = match tokens[0] {
            Token::Number(number) => number,
            _ => {continue;}
        };

        match tokens[1] {
            Token::OpMul => tokens[1], 
            _ => {continue;}
        };

        let number2 = match tokens[2] {
            Token::Number(number) => number, 
            _ => {continue;}
        };

        return Some((Token::Number(number1 * number2), i..i+3));
    }


    None
}

fn try_reduce_range_v2(list: &mut Vec<Token>, range: Range<usize>) -> bool {

    if list.len() < 3 {
        return false;
    }

    for i in range.start..range.end-2 {
        let tokens = &list[i..i+2];
        match tokens[0] {
            Token::GroupStart => {},
            _ => {continue;}
        }

        let mut count = 1;
        let mut end = 0;
        for j in i+1..range.end {
            if list[j] == Token::GroupStart {
                count += 1;
            }
            
            if list[j] == Token::GroupEnd {
                count -= 1;
            }

            if count == 0 {
                end = j;
                break;
            };

        };

        if try_reduce_range_v2(list, i+1..end) {
            return true;
        }
    }

    if let Some((new_token, range_to_replace)) = try_single_reduce_single_expression(&list[range.clone()] ) {
        //let range_to_replace = range_to_replace;
        let range_to_replace = range.start+range_to_replace.start..range.start+range_to_replace.end;
        list.drain(range_to_replace.clone());
        list.insert(range_to_replace.start, new_token);
        return true;
    }


    false
}

fn try_reduce_v2(list: &mut Vec<Token>) -> bool {
    return try_reduce_range_v2(list, 0..list.len());
    
}

fn parse_expression_v2(line: &str) -> i64 {

    let re = regex::Regex::new(r"\(|\)|\s+|\w+|\+|\*").unwrap();

    let mut it = re.find_iter(line);

    let mut list = Vec::<Token>::new();


    while let Some(token_as_str) = it.next().and_then(|token| Some(token.as_str())) {

        let token: Token = token_as_str.into();
        //let expression = Expression::Token(token);
        
        
        if let Token::Whitespace = token {
            continue;
        }

        //println!("'{}'", token_as_str);

        list.push(token);
    }

    println!();
    println!("{}", line);

    while try_reduce_v2(&mut list) { 
        println!("{:?}", list);
    }

    debug_assert!(list.len() == 1);
    
    let number = match list.pop().unwrap() {
        Token::Number(number) => number,
        _ => panic!(),
    };

    println!("{} -> {}", line, number);

    number
}


fn part2(input: &Vec<String>) -> String {

    let answer = input.iter()
        .map(|x| parse_expression_v2(x))
        .reduce(|a, b| a + b)
        .unwrap();

    return answer.to_string();
}

fn main() { 

    lib_aoc::run_with_test("day18", Some(part1), Some(part2));

}

