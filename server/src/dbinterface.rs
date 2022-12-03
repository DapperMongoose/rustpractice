use std::error::Error;
use std::fmt;

const READ_QUERY: &str = "SELECT * FROM COUNTER;";
const UPDATE_DB_QUERY: &str ="UPDATE COUNTER SET count = ?;";

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


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
