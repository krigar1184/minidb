use std::io::prelude::*;
extern crate lazy_static;

mod command;
mod statement;
mod table;

use command::Command;

fn main() {
    println!("Welcome to MyPoorDB version 0.1");
    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let command = Command::from_input();
        command.run().unwrap_or_else(|err| {
            eprintln!("An error occurred: {}", err);
        });
    }
}
