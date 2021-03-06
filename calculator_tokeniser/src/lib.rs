use std::collections::VecDeque;

// e.g + is binary, - is binary or right, ! is left
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum OpType {
    Binary,
    Right,
    Left,
}

// Association Precedence.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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
    Fac,
    Sin,
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

pub fn factorial(n: u64) -> u64 {
    if n < 1 {
        1
    }
    else {
        n*factorial(n-1)
    }
}

fn isdigit(some_char: &str) -> bool {
    some_char.to_owned().parse::<u8>().ok() != None
}

fn get_lit(token_string: &str, token_ptr: &mut usize, lit_stack: &mut VecDeque<Token>) {
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
                        if isdigit(prev_char) || prev_char == ")" {
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
        lit_stack.push_back(Token::Literal(token_string.get(num_start..*token_ptr).unwrap().to_owned().parse().unwrap()));
    }
}

fn get_op_or_sep(token_string: &str, token_ptr: &mut usize, op_stack: &mut VecDeque<Token>) {
    if *token_ptr < token_string.len() {
        let cur_char = token_string.get(*token_ptr..*token_ptr+1).unwrap();
        op_stack.push_back(match cur_char {
            "-" => Token::Operator(Operator::Sub),
            "+" => Token::Operator(Operator::Add),
            "/" => Token::Operator(Operator::Div),
            "*" => Token::Operator(Operator::Mul),
            "^" => Token::Operator(Operator::Pow),
            "!" => Token::Operator(Operator::Fac),
            "(" => Token::Separator(Separator::LBrac),
            ")" => Token::Separator(Separator::RBrac),
            _ => {
                let function = token_string.get(*token_ptr..*token_ptr+3).unwrap();
                *token_ptr+=2;
                match function {
                    "sin" => Token::Operator(Operator::Sin),
                    _ => panic!("Bad operator: {}, stack: {:?}", function, op_stack),
                }
            },
        });
    }
}

fn get_precedence(operator: &Operator) -> u8 {
    match *operator {
        Operator::Add => 1,
        Operator::Sub => 1,
        Operator::Div => 2,
        Operator::Mul => 2,
        Operator::Pow => 3,
        Operator::Fac => 4,
        Operator::Sin => 5,
    }
}

fn get_op_dir(operator: &Operator) -> AscDir {
    match *operator {
        Operator::Add => AscDir::Left,
        Operator::Sub => AscDir::Left,
        Operator::Div => AscDir::Left,
        Operator::Mul => AscDir::Left,
        Operator::Pow => AscDir::Right,
        Operator::Fac => AscDir::Left,
        Operator::Sin => AscDir::Left,
    }
}

fn get_op_type(operator: &Operator) -> OpType {
    match *operator {
        Operator::Add => OpType::Binary,
        Operator::Sub => OpType::Binary,
        Operator::Div => OpType::Binary,
        Operator::Mul => OpType::Binary,
        Operator::Pow => OpType::Binary,
        Operator::Fac => OpType::Left,
        Operator::Sin => OpType::Right,
    }
}

fn process_compute_stacks(vals_compute_stack: &mut Vec<f64>, ops_compute_stack: &mut Vec<Operator>) -> f64 {
    let mut result = vals_compute_stack.pop().unwrap();
    while let Some(op) = ops_compute_stack.pop() {
        match op {
            Operator::Add => result += vals_compute_stack.pop().unwrap(),
            Operator::Sub => result -= vals_compute_stack.pop().unwrap(),
            Operator::Div => result /= vals_compute_stack.pop().unwrap(),
            Operator::Mul => result *= vals_compute_stack.pop().unwrap(),
            Operator::Sin => result = result.sin(),
            _ => panic!("this shouldn't happen!")
        }
    }
    result
}

fn reduce(vals_stack: &mut Vec<f64>, ops_stack: &mut Vec<Token>, min_prec: u8) {
    let mut ops_compute_stack = Vec::new();
    let mut vals_compute_stack = Vec::new();

    let mut max_op_prec = 0;
    let mut found_rbrac = false;

    while let Some(token) = ops_stack.pop() {
        if let Token::Operator(op) = token {
            let cur_op_prec = get_precedence(&op);
            if cur_op_prec <= min_prec {
                vals_compute_stack.push(vals_stack.pop().unwrap());
                vals_stack.push(process_compute_stacks(&mut vals_compute_stack, &mut ops_compute_stack));
                ops_stack.push(Token::Operator(op));
                break;
            }
            if cur_op_prec < max_op_prec {
                vals_compute_stack.push(vals_stack.pop().unwrap());
                vals_stack.push(process_compute_stacks(&mut vals_compute_stack, &mut ops_compute_stack));
            }
            match get_op_dir(&op) {
                AscDir::Left => {
                    ops_compute_stack.push(op);
                    if get_op_type(&op) == OpType::Binary {
                        vals_compute_stack.push(vals_stack.pop().unwrap());
                    }
                },
                AscDir::Right => {
                    match op {
                        Operator::Pow => {
                            let val2 = vals_stack.pop().unwrap();
                            let val1 = vals_stack.pop().unwrap();
                            vals_stack.push(val1.powf(val2));
                        },
                        _ => panic!("Operator is not a right associative: {:?}", op),
                    }
                },
            }
            max_op_prec = cur_op_prec;
        }
        else if let Token::Separator(Separator::LBrac) = token {
            vals_compute_stack.push(vals_stack.pop().unwrap());
            vals_stack.push(process_compute_stacks(&mut vals_compute_stack, &mut ops_compute_stack));
            if !found_rbrac {
                ops_stack.push(Token::Separator(Separator::LBrac));
            }
            break;
        }
        else if let Token::Separator(Separator::RBrac) = token {
            found_rbrac = true;
        }
    }

    if let Some(val) = vals_stack.pop() {
        vals_compute_stack.push(val);
    }
    vals_stack.push(process_compute_stacks(&mut vals_compute_stack, &mut ops_compute_stack));
}

pub fn compute_string(token_string: &str) -> f64 {
    // Step 1: Tokenize.
    let mut token_queue = VecDeque::new();
    
    let mut token_ptr = 0;

    while token_ptr < token_string.len() {
        get_lit(token_string, &mut token_ptr, &mut token_queue);
        get_op_or_sep(token_string, &mut token_ptr, &mut token_queue);

        token_ptr += 1;
    }

    // Step 2: Compute.
    let mut cur_max_prec = 0;

    let mut vals_stack = Vec::new();
    let mut ops_stack = Vec::new();
    while let Some(token) = token_queue.pop_front() {
        match token {
            Token::Literal(val) => vals_stack.push(val),
            Token::Operator(op) => {
                let op_prec = get_precedence(&op);
                if op_prec < cur_max_prec {
                    reduce(&mut vals_stack, &mut ops_stack, op_prec);
                }
                match get_op_type(&op) {
                    OpType::Left => {
                        // Handles factorial.
                        match op {
                            Operator::Fac => {
                                let result = factorial(vals_stack.pop().unwrap() as u64);
                                vals_stack.push(result as f64);
                            },
                            _ => panic!("Not a left optype."),
                        }
                    },
                    _ => {
                        ops_stack.push(Token::Operator(op));
                        cur_max_prec = op_prec;
                    },
                }
            },
            Token::Separator(separator) => {
                match separator {
                    Separator::LBrac => {
                        ops_stack.push(Token::Separator(Separator::LBrac));
                        cur_max_prec = 0;
                    },
                    Separator::RBrac => {
                        ops_stack.push(Token::Separator(Separator::RBrac));
                        reduce(&mut vals_stack, &mut ops_stack, 0);
                        if ops_stack.len() > 0 {
                            if let Some(Token::Operator(last_op)) = ops_stack.get(ops_stack.len()-1) {
                                cur_max_prec = get_precedence(last_op);
                            }
                            else if let Some(Token::Separator(Separator::LBrac)) = ops_stack.get(ops_stack.len()-1) {
                                cur_max_prec = 0;
                            }
                        }
                    }
                }
            },
        }
    }

    reduce(&mut vals_stack, &mut ops_stack, 0);
    vals_stack[0]
}

use std::os::raw::{c_char};
use std::ffi::{CStr};

#[no_mangle]
pub extern fn calculate_string(to: *const c_char) -> f64 {
    let c_str = unsafe { CStr::from_ptr(to) };
    let calculation = match c_str.to_str() {
        Err(_) => "1",
        Ok(string) => string,
    };

    compute_string(calculation)
}


#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring, jdouble};

    #[no_mangle]
    pub unsafe extern fn Java_com_example_calculator_RustCalculator_calculateString(env: JNIEnv, _: JClass, java_pattern: JString) -> jdouble {
        calculate_string(env.get_string(java_pattern).expect("invalid pattern string").as_ptr())
    }
}


#[cfg(test)]
mod tests {
    use super::{compute_string, factorial};

    #[test]
    fn test_calculations() {
        assert_eq!(compute_string("1+2-3/4+5"), 1.+2.-3./4.+5.);
        assert_eq!(compute_string("1+2-3/4+5"), 1.+2.-3./4.+5.);
        assert_eq!(compute_string("3-2^3^4E-4"), 3.-2f64.powf(3f64.powf(4E-4)));
        assert_eq!(compute_string("5E5/-6E10"), 5E5/-6E10);
        assert_eq!(compute_string("3/5!+2^3E-4"), 3./factorial(5) as f64 +2f64.powf(3E-4));
        assert_eq!(compute_string("(2-3)*(4+5E-4)+3"), (2.-3.)*(4.+5E-4)+3.);
        assert_eq!(compute_string("(3!/(4+5.2E8))^3-2E-11"), (factorial(3) as f64 /(4.+5.2E8)).powf(3.)-2E-11);
        assert_eq!(compute_string("2*(sin(2+3)-4)"), 2.*((2f64+3f64).sin()-4.));
        assert_eq!(compute_string("sin((2+3)-4)"), ((2f64+3f64)-4.).sin());
        assert_eq!(compute_string("2*(3*4+5)"), 2.*(3.*4.+5.));
        assert_eq!(compute_string("3333333333"), 3333333333.);
        assert_eq!(compute_string("2.5-0.3E1"), 2.5-0.3E1);
        assert_eq!(compute_string("0/0").is_nan(), (0f64/0.).is_nan());
        assert_eq!(compute_string("3-2^3^sin(4E-4)"), 3.-2f64.powf(3f64.powf(4E-4f64.sin())));
    }
}
