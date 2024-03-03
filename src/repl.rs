use std::{boxed::Box, process};

#[derive(Debug)]
pub enum ReplError {
    BadRowNumber,
    RowNumberNotFound,
    BadKey,
    BadUsername,
    BadEmail,
    TableIndexError,
}

#[derive(Debug)]
pub enum ReplMessage {
    Executed,
    Fetched(String),
}

pub type InsertArgs<'a> = Box<dyn Iterator<Item = &'a str> + 'a>;
pub enum StatementType<'a> {
    Insert { args: InsertArgs<'a> },
    Select { args: InsertArgs<'a> },
}

pub fn do_meta_command(command: &String) -> Result<(), ()> {
    match command.as_str().trim() {
        ".exit" => {
            process::exit(0);
        }
        _ => Err(()),
    }
}
