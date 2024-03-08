extern crate processor;
extern crate table;

use std::{
    io::{self, Write},
    process,
};

use table::Table;

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn do_meta_command(command: &String) -> Result<(), ()> {
    match command.as_str().trim() {
        ".exit" => {
            process::exit(0);
        }
        _ => Err(()),
    }
}

pub fn run(table: &mut Table) {
    loop {
        let mut input_buffer = String::new();
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
        } else {
            match processor::prepare_statement(&input_buffer) {
                Ok(statement) => {
                    processor::execute_statement(statement, table);
                }
                Err(_) => {
                    println!("Unrecognized keyword at start of {}", input_buffer);
                }
            }
        }
    }
}
