use std::io::Read;
use std::{env::args, fs::File};

#[derive(Debug)]
enum Token {
    NUMBER(i32),
    INCREMENT,
    DECREMENT,
    SQUARE,
    PLUS,
    MINUS,
    PUSH,
    DUMP,
    OUT,
}

fn identify_token(input: &str) -> Token {
    match input {
        "push" => Token::PUSH,
        "inc" => Token::INCREMENT,
        "dec" => Token::DECREMENT,
        "sqr" => Token::SQUARE,
        "add" => Token::PLUS,
        "sub" => Token::MINUS,
        "dump" => Token::DUMP,
        "out" => Token::OUT,
        x if x.parse::<i32>().is_ok() => {
            let x_as_i32: i32 = x.parse::<i32>().unwrap();
            Token::NUMBER(x_as_i32)
        }
        x => {
            panic!("Unexpected token {}", x);
        }
    }
}

fn parse_instruction(instruction: &str, state: &mut Vec<i32>) {
    let words: Vec<&str> = instruction.trim().split(" ").collect();
    let tokenized: Vec<Token> = words.into_iter().map(identify_token).collect();

    if tokenized.len() > 2 {
        panic!("Expression block can't contain more than 2 tokens");
    }
    if tokenized.len() == 1 {
        match &tokenized[0] {
            Token::INCREMENT => {
                let on_stack = state.pop().expect("Stack underflow");
                state.push(on_stack + 1);
            }
            Token::DECREMENT => {
                let on_stack = state.pop().expect("Stack underflow");
                state.push(on_stack - 1);
            }
            Token::MINUS => {
                let x = state.pop().expect("Stack underflow");
                let y = state.pop().expect("Stack underflow");
                state.push(y - x);
            }
            Token::PLUS => {
                let x = state.pop().expect("Stack underflow");
                let y = state.pop().expect("Stack underflow");
                state.push(x + y);
            }
            Token::SQUARE => {
                let on_stack = state.pop().expect("Stack underflow");
                state.push(on_stack * on_stack);
            }

            Token::DUMP => {
                let on_stack = state.pop().expect("Stack underflow");
                println!("{}", on_stack);
            }
            Token::OUT => {
                let on_stack = state.last();
                println!("{}", on_stack.expect("Stack underflow"));
            }
            x => {
                panic!("Unexpected syntax at token {:?}", x);
            }
        }
    }
    if tokenized.len() == 2 {
        match &tokenized[..] {
            [Token::PUSH, Token::NUMBER(x)] => {
                state.push(*x);
            }
            _ => {
                panic!("Syntax error");
            }
        }
    }
}

fn main() {
    let arg_vec: Vec<String> = args().skip(1).collect();
    let arg_str_vec: Vec<&str> = arg_vec.iter().map(|v| &v[..]).collect();
    let file_path: String;

    match arg_str_vec[..] {
        [x] => {
            file_path = x.to_string();
        }
        _ => {
            println!("USAGE:\n dirtycat <FILE_PATH>");
            panic!("Invalid usage");
        }
    }

    let mut file = File::open(file_path).expect("Can't open file with given path");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Can't read file contents");

    contents = contents.replace("\n", "");

    let mut state: Vec<i32> = Vec::new();
    let mut instructions: Vec<&str> = contents.split(";").collect();
    instructions.pop();
    for inst in instructions {
        parse_instruction(inst, &mut state);
    }

    return ();
}
