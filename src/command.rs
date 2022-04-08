use std::io::stdin;
use crate::statement::{Statement, StatementType, InvalidStatementError};

#[derive(Debug)]
pub struct Command {
    data: String,
}

impl Command {
    pub fn from_input() -> Self {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer = String::from(buffer.trim_matches('\n'));
        Command { data: buffer }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_meta_command() {
            return self.do_meta_command()
        }

        let lowercased = self.data.to_lowercase();
        let (stmt_type, payload) = lowercased.trim().split_at(6);
        let stmt = match stmt_type {
            "insert" => <dyn Statement>::new(StatementType::INSERT, payload),
            "select" => <dyn Statement>::new(StatementType::SELECT, payload),
            _ => return Err(Box::new(InvalidStatementError{stmt: String::from(self.data.as_str())})),
        };

        match stmt.prepare() {
            Ok(_v) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn is_meta_command(&self) -> bool {
        self.data.starts_with('.')
    }

    fn do_meta_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.data.as_str() {
            ".exit" => {
                println!("See you later, alligator");
                std::process::exit(0);
            },
            value => {
                eprintln!("Unrecognized command {}", value);
                Ok(())
            }
        }
    }
}
