extern crate processor;
extern crate repl;
extern crate table;

use repl::Repl;
use table::Table;

fn main() {
    println!("WELCOME TO SHITTY SQL REPL");
    let repl = Repl::new();
    repl.run();
}
