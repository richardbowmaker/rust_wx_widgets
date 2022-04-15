
use std::{fs::File, io};
use std::io::ErrorKind;
use core::num::ParseIntError;
use std::fmt;

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
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        let s = format!("io::Error {}", err);
        AppError { kind: AppErrorT::MyIoError(err.kind()), description: s }
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

pub fn errors_main() {


    match file_open_errs1("nosuchfile.txt") {
        Ok(_) => println!("File opened OK"),
        Err(err) => println!("errors::main() {}", &err.description)
    }

    match file_open_errs2("nosuchfile.txt") {
        Some(_) => println!("File opened OK"),
        None => println!("errors::main()")
    }


}

fn file_open_errs1(filename: &str) -> Result<File, AppError> {

    let f = File::open(filename)?;
    // let file = error_catch!(File::open(&filename), &format!("File open error {}", &filename));
    // let my_int: i32 = "xyz".parse()?;

    Ok(f)
}

fn file_open_errs2(filename: &str) -> Option<File> {

    let f = File::open(filename).ok();

    f
}