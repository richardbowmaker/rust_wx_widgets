
use std::{fs::File, io};
use std::io::ErrorKind;
use core::num::ParseIntError;
use std::fmt;
use std::io::Error;

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

pub enum AppErrorT {
    MyIoError(ErrorKind),
    MyParseIntError,
}

pub struct AppError {
    kind: AppErrorT,
    description: String,
}

// convert from io::Error to MyError
impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        let s = format!("io::Error {}", err);
        AppError { kind: AppErrorT::MyIoError(err.kind()), description : s }
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        let s = format!("ParseIntError {} {} {}", err, file!(), line!());
        AppError { kind: AppErrorT::MyParseIntError, description: s }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppError, {}", &self.description)
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
