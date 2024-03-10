extern crate processor;
extern crate repl;
extern crate table;

use table::Table;

fn main() {
    println!("WELCOME TO SHITTY SQL REPL");
    let mut table = Table::open(String::from("test.db"));
    repl::run(&mut table);
}
