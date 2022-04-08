use lazy_regex::regex_captures;

use crate::table::Row;

#[derive(Debug)]
pub struct InvalidStatementError {
    pub stmt: Option<String>,
}

impl std::fmt::Display for InvalidStatementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.stmt {
            Some(v) => write!(f, "Unrecognized statement: {}", v),
            None => write!(f, "Unrecognized statement"),
        }
    }
}

type Result<T> = std::result::Result<T, InvalidStatementError>;

impl std::error::Error for InvalidStatementError {}

#[derive(Debug)]
pub enum StatementType {
    INSERT,
    SELECT,
}

pub trait Statement<'a> {
    fn execute(&self) -> Result<()>;
    fn prepare(&self) -> Result<()>;
}

impl<'a> dyn Statement<'a> {
    pub fn new(r#type: StatementType, payload: &'a str) -> Box<dyn Statement<'a> + 'a> {
        let stmt: Box<dyn Statement<'a> + 'a> = match r#type {
            StatementType::SELECT => Box::new(SelectStatement::new(payload).unwrap()),
            StatementType::INSERT => Box::new(InsertStatement::new(payload).unwrap()),
        };
        stmt
    }
}

pub struct SelectStatement<'a> {
    rows_to_select: Vec<Row<'a>>,
}

impl<'a> SelectStatement<'a> {
    pub fn new(payload: &'a str) -> Result<Self> {
        Ok(SelectStatement {rows_to_select: vec![]})
    }
}

impl<'a> Statement<'a> for SelectStatement<'a> {
    fn execute(&self) -> Result<()> {
        Ok(())
    }

    fn prepare(&self) -> Result<()> {
        Ok(())
    }
}

pub struct InsertStatement<'a> {
    rows_to_insert: Vec<Row<'a>>,
}

impl<'a> InsertStatement<'a> {
    pub fn new(payload: &'a str) -> Result<Self> {
        if let Some((_, id, username, email)) = regex_captures!(r"(\w+)\s*(\w+)\s*(\w+)\s*", payload) {
            let parsed_id: u64 = id.parse().unwrap();
            let row = Row::new(parsed_id, username, email);
            return Ok(InsertStatement {rows_to_insert: vec![row]})
        }

        Err(InvalidStatementError{stmt: None})
    }
}

impl<'a> Statement<'a> for InsertStatement<'a> {
    fn execute(&self) -> Result<()> {
        Ok(())
    }

    fn prepare(&self) -> Result<()> {
        Ok(())
    }
}
