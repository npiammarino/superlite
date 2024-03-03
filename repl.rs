use std::{
    boxed::Box,
    io::{self, Write},
    process,
};

const PAGE_SIZE: usize = 10;

#[derive(Debug)]
enum ReplError {
    BadRowNumber,
    RowNumberNotFound,
    BadKey,
    BadUsername,
    BadEmail,
    TableIndexError,
}
use crate::ReplError::*;
#[derive(Debug)]
enum ReplMessage {
    Executed,
    Fetched(Row),
}
use crate::ReplMessage::*;

type InsertArgs<'a> = Box<dyn Iterator<Item = &'a str> + 'a>;
enum StatementType<'a> {
    Insert { args: InsertArgs<'a> },
    Select { args: InsertArgs<'a> },
}
use crate::StatementType::*;

// -- Table stuff for in memory impl
#[derive(Debug, Clone)]
struct Row {
    id: usize,
    username: String,
    email: String, // could make special type for this
}
#[derive(Debug)]
struct Page {
    // need to serialize eventually
    max_size: usize,
    rows: Vec<Row>,
}
#[derive(Debug)]
struct Table {
    num_rows: usize,
    pages: Vec<Page>, // need to serialize eventually
}

impl Table {
    fn push_row(&mut self, row: Row) {
        if self.num_rows % PAGE_SIZE == 0 {
            self.pages.push(Page {
                max_size: PAGE_SIZE,
                rows: vec![],
            });
        }

        let page_index = self.num_rows / PAGE_SIZE;
        self.pages[page_index].rows.push(row);
        self.num_rows += 1;
    }
    fn get_row(&self, row_number: usize) -> Result<Row, ReplError> {
        if row_number + 1 > self.num_rows {
            return Err(TableIndexError);
        }

        let (page_index, row_index) = (row_number / PAGE_SIZE, row_number % PAGE_SIZE);
        Ok(self.pages[page_index].rows[row_index].clone())
    }
}

// ------------

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut table = Table {
        num_rows: 0,
        pages: vec![],
    };
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

fn do_meta_command(command: &String, table: &Table) -> Result<(), ()> {
    match command.as_str().trim() {
        ".exit" => {
            println!("{table:?}");
            process::exit(0);
        }
        _ => Err(()),
    }
}

fn prepare_statement(input_buffer: &String) -> Result<StatementType, ()> {
    let mut statement = input_buffer.as_str().split_whitespace();
    let stype = statement.next().unwrap();
    match stype {
        "insert" => Ok(Insert {
            args: Box::new(statement),
        }),
        "select" => Ok(Select {
            args: Box::new(statement),
        }),
        _ => Err(()),
    }
}

fn execute_statement(statement: StatementType, table: &mut Table) {
    let res: Result<_, ReplError> = match statement {
        Insert { args } => execute_insert(args, table),
        Select { args } => execute_select(args, table),
    };

    let message = match res {
        Ok(msg) => format!("{msg:?}"),
        Err(e) => format!("Error: {e:?}"),
    };

    println!("{message}")
}

fn execute_select(
    mut statement_args: InsertArgs,
    table: &mut Table,
) -> Result<ReplMessage, ReplError> {
    match statement_args.next() {
        Some(s) => match s.parse::<usize>() {
            Ok(n) => match table.get_row(n - 1) {
                Ok(row) => Ok(Fetched(row)),
                Err(e) => Err(e),
            },
            _ => Err(BadRowNumber),
        },
        _ => Err(RowNumberNotFound),
    }
}

fn execute_insert(
    mut statement_args: InsertArgs,
    table: &mut Table,
) -> Result<ReplMessage, ReplError> {
    // probably do zip with defn to generalize
    let id: usize = match statement_args.next() {
        Some(s) => match s.parse() {
            Ok(n) => n,
            _ => return Err(BadKey),
        },
        _ => {
            return Err(BadKey);
        }
    };

    let username = match statement_args.next() {
        Some(s) => s.to_string(),
        _ => {
            return Err(BadUsername);
        }
    };

    let email = match statement_args.next() {
        Some(s) => s.to_string(),
        _ => {
            return Err(BadEmail);
        }
    };

    let row = Row {
        id,
        username,
        email,
    };

    table.push_row(row);
    Ok(Executed)
}
