extern crate repl;
extern crate table;

use repl::{
    InsertArgs,
    ReplError::{self, *},
    ReplMessage::{self, *},
    StatementType::{self, *},
};

use table::{Row, Table};

pub fn prepare_statement(input_buffer: &String) -> Result<StatementType, ()> {
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

pub fn execute_statement(statement: StatementType, table: &mut Table) {
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
                Ok(row) => Ok(Fetched(format!("{row:?}"))),
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

    table.push_row(Row::build(id, username, email));
    Ok(Executed)
}
