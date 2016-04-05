pub mod dotnet_config;

use std::fmt;
use std::fs::File;
use std::io::Read;

pub struct ParseFileResult {
    pub is_good: bool,
    pub cs_name: String,
    pub server_name: String,
    pub db_name: String
}

pub enum Error {
    CannotOpenFile,
    CannotReadFile,
    CannotFindConnectionString,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::CannotOpenFile => { return write!(f, "Cannot open file"); }
            Error::CannotReadFile => { return write!(f, "Cannot read file"); }
            Error::CannotFindConnectionString => { return write!(f, "Cannot find connection string"); }
        }
    }
}

pub enum Parser {
    DotNet
}

pub fn parse_file(file_path: &str, parser: Parser) -> Result<Vec<ParseFileResult>, Error> {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut string_content = String::new();
            match file.read_to_string(&mut string_content) {
                Ok(_) => {
                    match parser {
                        Parser::DotNet => { return dotnet_config::parse(&string_content); }
                    }
                }
                Err (_) => { return Err(Error::CannotReadFile); }
            }
        }
        Err(_) => {
            return Err(Error::CannotOpenFile);
        }
    }
}
