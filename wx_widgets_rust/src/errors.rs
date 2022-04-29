
use std::{fs::File, io};
use std::io::ErrorKind;
use core::num::ParseIntError;
use std::fmt;
use std::io::Error;
use regex::Regex;


macro_rules! error_catch{
        // match like arm for macro
           ($call:expr,$mess:expr)=>{
               {
                   match $call {
                        Ok(res) => res,
                        Err(err) => {
                            println!("{} {} {} {}", &$mess, err, file!(), line!());
                            return Err(AppError::from(err))
                        }
                   }
               }
           }
        }


pub struct AppError {
    kind: AppErrorKind,
    description: String,
}

pub enum AppErrorKind {
    GeneralError,
    IoError(ErrorKind),
    ParseIntError,
    RegexError,
}

impl AppError {
    pub fn new(text : &str) -> Self {
        Self { kind : AppErrorKind::GeneralError, description : String::from(text) }
    }
}

// convert from io::Error to MyError
impl From<io::Error> for AppError {
    fn from(err: Error) -> Self {
        let s = format!("{}, kind : {:?}", err, err.kind());
        AppError { kind: AppErrorKind::IoError(err.kind()), description : s }
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        let s = format!("{}", err);
        AppError { kind: AppErrorKind::ParseIntError, description: s }
    }
}

impl From<regex::Error> for AppError {
    fn from(err : regex::Error) -> Self {
        let s = format!("{}", err);
        AppError { kind: AppErrorKind::RegexError, description: s }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            AppErrorKind::GeneralError              => write!(f, "Error")?,
            AppErrorKind::IoError(kind)  => write!(f, "Parse integer error : {:?}", kind)?,
            AppErrorKind::ParseIntError             => write!(f, "Parse integer error")?,
            AppErrorKind::RegexError                => write!(f, "Regex error")?,
        }
        write!(f, " : {}", &self.description)
    }
}

// --------------------------------------------------
// example use of error propagation

pub fn main() {
    let to = try_open_3();
    match to {
        Ok(_) => {
            println!("opened OK");
            ()
        }
        Err(e) => {
            println!("open failed {}", e);
            ()
        }
    }
}

// verbose
pub fn try_open_1() -> Result<(), Error> {

    let f = File::open("foo.txt");

    match f {
        Ok(f) => Ok(()),
        Err(e) => Err(e),
    }
}

// ? operator
pub fn try_open_2() -> Result<(), Error> {

    let f = File::open("foo.txt")?;
    Ok(())
}

// error propagation and conversion to AppError using From trait
pub fn try_open_3() -> Result<(), AppError> {
    let f = File::open("foo.txt")?;
    Ok(())
}
