extern crate repl;

use repl::*;
use std::io::{self, Write};

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut table = Table::new();
    println!("WELCOME TO SHITTY SQL REPL");
    loop {
        let mut input_buffer = String::new();
        print_prompt();
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");

        if input_buffer.chars().next() == Some('.') {
            match do_meta_command(&input_buffer, &table) {
                Ok(_) => {}
                Err(_) => {
                    println!("Unrecognized command {}", input_buffer);
                }
            }
        } else {
            match prepare_statement(&input_buffer) {
                Ok(statement) => {
                    execute_statement(statement, &mut table);
                }
                Err(_) => {
                    println!("Unrecognized keyword at start of {}", input_buffer);
                }
            }
        }
    }
}
