use lib_aoc::*;

#[derive(Clone, Copy, PartialEq)]
enum Precedence {
    Equal,
    AddIsStronger
}

enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Number(i64),
}

#[derive(Clone, Copy, PartialEq)]
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

fn parse_expression(line: &str, precedence: Precedence) -> i64 {

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



fn parse_expression_v2(line: &str) -> i64 {

    let re = regex::Regex::new(r"\(|\)|\s+|\w+|\+|\*").unwrap();

    let mut it = re.find_iter(line);

    let mut expression_tree = None::<Expression>;
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
        .map(|x| parse_expression(x, Precedence::Equal))
        .reduce(|a, b| a + b)
        .unwrap();

    return answer.to_string();
}



fn part2(input: &Vec<String>) -> String {

    let answer = input.iter()
        .map(|x| parse_expression(x, Precedence::AddIsStronger))
        .reduce(|a, b| a + b)
        .unwrap();

    return answer.to_string();
}

fn main() { 

    lib_aoc::run_with_test("day18", Some(part1), Some(part2));

}

