use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
pub enum RedisValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>), // None for null bulk string
    Array(Option<Vec<RedisValue>>), // None for null array
}

#[derive(Debug)]
pub enum ParseError {
    IoError(io::Error),
    InvalidFormat(String),
    UnexpectedEof,
    InvalidInteger(String),
    InvalidLength(String),
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IoError(err)
    }
}

pub trait RedisParser {
    /// Parse a single Redis value from the reader
    fn parse<R: BufRead>(&self, reader: &mut R) -> Result<Option<RedisValue>, ParseError>;
    
    /// Parse multiple Redis values until EOF
    fn parse_stream<R: BufRead>(&self, reader: &mut R) -> Vec<Result<RedisValue, ParseError>> {
        let mut results = Vec::new();
        loop {
            match self.parse(reader) {
                Ok(Some(value)) => results.push(Ok(value)),
                Ok(None) => break, // EOF - clean exit
                Err(err) => {
                    results.push(Err(err));
                    break; // Error - stop parsing
                }
            }
        }
        results
    }
}

pub struct BasicRedisParser;

impl BasicRedisParser {
    pub fn new() -> Self {
        Self
    }
}

impl RedisParser for BasicRedisParser {
    fn parse<R: BufRead>(&self, reader: &mut R) -> Result<Option<RedisValue>, ParseError> {
        let mut line = String::new();
        let result = reader.read_line(&mut line).unwrap();
        if !line.ends_with("\r\n") {
            return Err(ParseError::InvalidFormat("Missing CRLF".to_string()));
        }
        let content = line.split_off(line.len() - 2);
        let redis_value = match content[0] {
            '+' => RedisValue::SimpleString(String::from(content[1..])),
        }
    }
}
