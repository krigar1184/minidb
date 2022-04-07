use crate::table::Row;

#[derive(Debug)]
pub struct InvalidStatementError {
    pub stmt: String,
}

impl std::fmt::Display for InvalidStatementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized statement: {}", self.stmt)
    }
}

type Result<T> = std::result::Result<T, InvalidStatementError>;

impl std::error::Error for InvalidStatementError {}

#[derive(Debug)]
pub enum StatementType {
    INSERT,
    SELECT,
}

pub trait Statement {
    fn execute(&self) -> Result<()>;
    fn prepare(&self) -> Result<()>;
}

impl dyn Statement {
    pub fn new(r#type: StatementType, payload: &str) -> Box<dyn Statement> {
        match r#type {
            StatementType::SELECT => Box::new(SelectStatement::new(payload)),
            StatementType::INSERT => Box::new(InsertStatement::new(payload)),
        }
    }
}

pub struct SelectStatement {
    rows_to_select: Vec<Row>,
}

impl SelectStatement {
    pub fn new(payload: &str) -> SelectStatement {
        SelectStatement {rows_to_select: vec![]}
    }
}

impl Statement for SelectStatement {
    fn execute(&self) -> Result<()> {
        Ok(())
    }

    fn prepare(&self) -> Result<()> {
        Ok(())
    }
}

pub struct InsertStatement {
    row_to_insert: Row,
}

impl InsertStatement {
    pub fn new(payload: &str) -> Self {
        let row = Row{};
        InsertStatement {row_to_insert: row}
    }
}

impl Statement for InsertStatement {
    fn execute(&self) -> Result<()> {
        Ok(())
    }

    fn prepare(&self) -> Result<()> {
        Ok(())
    }
}
