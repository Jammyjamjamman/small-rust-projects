enum OpTypes {
    Binary,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Operators {
    RBrac,
    Sub,
    Add,
    Mul,
    Div,
    Pow,
    LBrac,
}

fn isdigit(some_char: &str) -> bool {
    some_char.to_owned().parse::<u8>().ok() != None
}

fn get_lit(token_string: &str, token_ptr: &mut usize, lit_stack: &mut Vec<f64>) {
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
        lit_stack.push(token_string.get(num_start..*token_ptr).unwrap().to_owned().parse().unwrap());
        *token_ptr -= 1;
}

fn get_op(token_string: &str, token_ptr: &mut usize, op_stack: &mut Vec<Operators>) {
    let cur_char = token_string.get(*token_ptr..*token_ptr+1).unwrap();
    op_stack.push(match cur_char {
        "-" => Operators::Sub,
        "+" => Operators::Add,
        "/" => Operators::Div,
        "*" => Operators::Mul,
        _ => panic!("Bad operator."),
    });
}

fn compute_string(token_string: &str) {
    let mut op_stack = Vec::new();
    let mut lit_stack = Vec::new();

    let mut last_token_op = true;
    
    let mut token_ptr = 0;

    while token_ptr < token_string.len() {
        if last_token_op {
            get_lit(token_string, &mut token_ptr, &mut lit_stack);
            last_token_op = false;
        }
        else {
            get_op(token_string, &mut token_ptr, &mut op_stack);
            last_token_op = true;
        }
        token_ptr += 1;
    }
    println!("{:?} {:?}", op_stack, lit_stack);
}

fn main() {
    compute_string("-123.45-6E-23-12*3+-4");
}
