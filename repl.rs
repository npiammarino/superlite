use std::{
    io::{self, Write},
    process,
};

enum StatementType {
    Insert,
    Select,
}

enum ReplError {
    ArgumentError,
}

use crate::StatementType::*;

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut input_buffer = String::new();
    println!("WELCOME TO SHITTY SQL REPL");
    loop {
        print_prompt();
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");

        if input_buffer.chars().next() == Some('.') {
            match do_meta_command(&input_buffer) {
                Ok(_) => {}
                Err(_) => {
                    println!("Unrecognized command {}", input_buffer);
                }
            }
            input_buffer.clear();
        } else {
            match prepare_statement(&input_buffer) {
                Ok(statement) => {
                    execute_statement(statement);
                    println!("Executed.");
                    break;
                }
                Err(_) => {
                    println!("Unrecognized keyword at start of {}", input_buffer);
                    input_buffer.clear()
                }
            }
        }
    }
}

fn do_meta_command(command: &String) -> Result<(), ()> {
    match command.as_str().trim() {
        ".exit" => {
            process::exit(0);
        }
        _ => Err(()),
    }
}

fn prepare_statement(input_buffer: &String) -> Result<StatementType, ()> {
    let mut statement = input_buffer.as_str().split_whitespace();
    let stype = statement.next().unwrap();
    match stype {
        "insert" => Ok(Insert),
        "select" => Ok(Select),
        _ => Err(()),
    }
}

fn execute_statement(statement: StatementType) {
    match statement {
        Insert => println!("Do insert here..."),
        Select => println!("Do select here..."),
    }
}
