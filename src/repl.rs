extern crate processor;
extern crate table;

use std::{
    io::{self, Write},
    process,
};

use table::Table;

const DEFAULT_PROMPT: &str = "db";

fn print_prompt(prompt: &String) {
    print!("{prompt} > ");
    io::stdout().flush().unwrap();
}

pub struct Repl {
    table: Option<Table>,
    prompt: String,
}

impl<'a> Repl {
    pub fn new() -> Self {
        Repl {
            table: None,
            prompt: String::from(DEFAULT_PROMPT),
        }
    }

    pub fn open_table(&mut self, filename: String) {
        let new_table = Table::open(filename);
        self.table = Some(new_table);
    }

    pub fn get_table(&'a mut self) -> Result<&'a mut Table, ()> {
        if let Some(table) = self.table.as_mut() {
            return Ok(table);
        }
        Err(())
    }

    pub fn run(mut self) {
        loop {
            let mut input_buffer = String::new();
            print_prompt(&self.prompt);
            io::stdin()
                .read_line(&mut input_buffer)
                .expect("Failed to read line");

            if input_buffer.chars().next() == Some('.') {
                match self.do_meta_command(&input_buffer) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("Unrecognized command {}", input_buffer);
                    }
                }
            } else {
                match processor::prepare_statement(&input_buffer) {
                    Ok(statement) => match self.get_table() {
                        Ok(table) => {
                            processor::execute_statement(statement, table);
                        }
                        Err(_) => {
                            println!("ERROR");
                        }
                    },
                    Err(_) => {
                        println!("Unrecognized keyword at start of {}", input_buffer);
                    }
                }
            }
        }
    }

    fn do_meta_command(&mut self, command: &String) -> Result<(), ()> {
        let mut args = command.as_str().split_whitespace();
        match args.next().expect("handled in loop") {
            ".exit" => {
                if let Some(table) = self.table.as_mut() {
                    let _ = table.close();
                }
                process::exit(0);
            }
            ".open" => match args.next() {
                Some(filename) => {
                    let path = format! {"./data/{filename}.db"};
                    self.open_table(path);
                    self.prompt = format!("{DEFAULT_PROMPT} {filename}");
                    Ok(())
                }
                None => Err(()),
            },
            _ => Err(()),
        }
    }
}
