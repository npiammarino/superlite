extern crate processor;
extern crate repl;
extern crate table;

use table::Table;

fn main() {
    let mut table = Table::new();
    println!("WELCOME TO SHITTY SQL REPL");

    repl::run(&mut table);
}
