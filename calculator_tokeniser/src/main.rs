enum OpType {
    Binary,
    Right,
    Left,
}

enum AscDir {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Operator {
    Sub,
    Add,
    Mul,
    Div,
    Pow,
    Fac
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Separator {
    LBrac,
    RBrac,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Literal(f64),
    Operator(Operator),
    Separator(Separator),
}

fn isdigit(some_char: &str) -> bool {
    some_char.to_owned().parse::<u8>().ok() != None
}

fn get_lit(token_string: &str, token_ptr: &mut usize, lit_stack: &mut Vec<Token>) {
        let num_start = *token_ptr;

        while {
            *token_ptr < token_string.len()
            && {
                let next_char = token_string.get(*token_ptr..*token_ptr+1).unwrap();
                if isdigit(next_char) {
                    true
                }
                else if next_char == "-" {
                    if *token_ptr == 0 {
                        true
                    }
                    else {
                        if let Some(prev_char) = token_string.get(*token_ptr-1..*token_ptr) {
                            if isdigit(prev_char) {
                                false
                            }
                            else {
                                true
                            }
                        }
                        else {
                            true
                        }
                    }
                }
                else if next_char == "." || next_char == "E" {
                    true
                }
                else {
                    false
            }
        }
        } { *token_ptr += 1; }
        if num_start != *token_ptr {
            lit_stack.push(Token::Literal(token_string.get(num_start..*token_ptr).unwrap().to_owned().parse().unwrap()));
        }
}

fn get_op_or_sep(token_string: &str, token_ptr: &mut usize, op_stack: &mut Vec<Token>) {
    if *token_ptr < token_string.len() {
        let cur_char = token_string.get(*token_ptr..*token_ptr+1).unwrap();
        op_stack.push(match cur_char {
            "-" => Token::Operator(Operator::Sub),
            "+" => Token::Operator(Operator::Add),
            "/" => Token::Operator(Operator::Div),
            "*" => Token::Operator(Operator::Mul),
            "^" => Token::Operator(Operator::Pow),
            "!" => Token::Operator(Operator::Fac),
            "(" => Token::Separator(Separator::LBrac),
            ")" => Token::Separator(Separator::RBrac),
            _ => panic!("Bad operator."),
        });
    }
}

fn get_precedence(token: &Token) -> u8 {
    match token {
        Token::Separator(Separator::RBrac) => 0,
        Token::Operator(Operator::Add) => 1,
        Token::Operator(Operator::Sub) => 2,
        Token::Operator(Operator::Div) => 3,
        Token::Operator(Operator::Mul) => 4,
        Token::Operator(Operator::Pow) => 5,
        Token::Operator(Operator::Fac) => 6,
        Token::Separator(Separator::LBrac) => 7,
        _ => 8
    }
}

fn reduce(compute_stack: &mut Vec<Token>) {
    let mut next_op = Operator::Sub;
    let mut lits = Vec::new();

    while let Some(token) = compute_stack.pop() {
        match token {
            Token::Literal(val) => lits.push(val),
            Token::Operator(op) => next_op = op,
            _ => (),
        }
        // not good test.
        if lits.len() == 2 {
            match next_op {
                Operator::Add => compute_stack.push(Token::Literal(lits.pop().unwrap() + lits.pop().unwrap())),
                Operator::Sub => compute_stack.push(Token::Literal(lits.pop().unwrap() - lits.pop().unwrap())),
                _ => unimplemented!("implement other operators.")
            }
        }
    }
    compute_stack.push(Token::Literal(lits.pop().unwrap()));
}

fn compute_string(token_string: &str) {
    // Step 1: Tokenize.
    let mut token_stack = Vec::new();
    
    let mut token_ptr = 0;

    while token_ptr < token_string.len() {
        get_lit(token_string, &mut token_ptr, &mut token_stack);
        get_op_or_sep(token_string, &mut token_ptr, &mut token_stack);

        token_ptr += 1;
    }
    println!("{:?}", token_stack);

    // Step 2: Compute
    token_stack.reverse();
    let mut compute_stack = Vec::new();
    
    let mut last_op = Token::Separator(Separator::RBrac);
    while let Some(next_token) = token_stack.pop() {
        if let Token::Literal(_) = next_token {
            compute_stack.push(next_token);
        }
        else {
            if get_precedence(&last_op) <= get_precedence(&next_token) {
                last_op = next_token;
                compute_stack.push(next_token);
            }
            else {
                reduce(&mut compute_stack);
                last_op = next_token;
                compute_stack.push(next_token);
            }
        }
    }
    println!("{:?}", compute_stack);
    reduce(&mut compute_stack);
    println!("final: {:?}", compute_stack);
}

fn main() {
    compute_string("4-5-3");
}
