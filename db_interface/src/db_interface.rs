use std::error::Error;
use std::fmt;

const READ_QUERY: &str = "SELECT * FROM COUNTER;";

#[derive(Debug)]
pub struct DBError {
    details: String
}

impl DBError {
    fn new(msg: &str) -> DBError {
        DBError{details: msg.to_string()}
    }
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DBError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn read_db() ->  Result<i64, DBError> {
    let connection: sqlite::Connection = sqlite::open("db.sq3").unwrap();
    let mut result = Vec::new();

    for row in connection
        .prepare(READ_QUERY)
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap()) {
        result.push(row.read::<i64, _>("COUNT"));
    };
    if result.len() > 1 {
        return Err(DBError::new("Too many rows found in database, possible corruption"));
    }

    Ok(result[0])
    }

pub fn increment() -> Result<(), DBError>{
    let connection: sqlite::Connection = sqlite::open("db.sq3").unwrap();
    let mut current = read_db().unwrap();
    current += 1;
    let query = format!("UPDATE COUNTER SET count = {current};");
    connection.execute(query).unwrap();
    Ok(())
}

pub fn decrement() -> Result<(), DBError> {
    let connection: sqlite::Connection = sqlite::open("db.sq3").unwrap();
    let mut current = read_db().unwrap();
    // Don't let the count be negative, just because.
    if current > 0{
        current -= 1;
        let query = format!("UPDATE COUNTER SET count = {current};");
        connection.execute(query).unwrap();
    }
    Ok(())
}

pub fn reset() -> Result<(), DBError> {
    let connection: sqlite::Connection = sqlite::open("db.sq3").unwrap();
    let query = format!("UPDATE COUNTER SET count = 0;");
    connection.execute(query).unwrap();
    Ok(())
}