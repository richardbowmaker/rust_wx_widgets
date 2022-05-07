

use std::fs::File;
use std::io::{prelude::*, BufReader};
use crate::errors::AppError;
use std::default::Default;
use std::fmt;



// E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\include\wx\msw

// -----------------------------------------------
// Method type
pub struct Method {
    is_virtual      : bool, 
    return_type     : String,
    is_ref          : bool,
    is_pointer      : bool,
    name            : String,
    is_constructor  : bool,
    is_destructor   : bool,
    arguments       : Vec<Argument>,
}

impl Default for Method {
    fn default() -> Self {
        Self {
            is_virtual      : false, 
            return_type     : String::from(""), 
            is_ref          : false, 
            is_pointer      : false, 
            name            : String::from(""), 
            is_constructor  : false,
            is_destructor   : false,
            arguments       : Vec::new(), } 
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_virtual { write!(f, "virtual ")?; }
        write!(f, "{}", &self.return_type)?;
        if self.is_ref { write!(f, "&")?; }
        if self.is_pointer { write!(f, "*")?; }
        write!(f, " {} (", &self.name)?;
        let mut sep = "";
        for arg in &self.arguments {
            write!(f, "{}{}", &sep, arg)?;
            sep = ", ";
        }
        write!(f, ")")?;
        if self.is_constructor { write!(f, " // constructor")?; }
        if self.is_destructor { write!(f, " // destructor")?; }
        Ok(())
    }
}

impl Method {
    fn set_name(&mut self, name : &str) {
        self.name = name.trim().to_owned();
    }

    fn set_return_type(&mut self, return_type : &str) {
        self.return_type = return_type.trim().to_owned();
    }

    fn add_argument(&mut self, arg : Argument) {
        self.arguments.push(arg);
    }
}

// -----------------------------------------------
// Argument type
//
// const wxPoint &pos= wxDefaultPosition

pub struct Argument {
    is_const        : bool, 
    is_ref          : bool,
    is_pointer      : bool,
    type_           : String,
    name            : String,
    default_value   : String,
}

impl Default for Argument {
    fn default() -> Self {
        Self {
            is_const        : false, 
            is_ref          : false, 
            is_pointer      : false, 
            type_           : String::from(""), 
            name            : String::from(""), 
            default_value   : String::from(""),} 
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_const { write!(f, "const ")?; }
        write!(f, "{} ", &self.type_)?;
        if self.is_ref { write!(f, "&")?; }
        if self.is_pointer { write!(f, "*")?; }
        write!(f, "{}", &self.name)?;
        if self.default_value.len() > 0 {
            write!(f, "={}", &self.default_value)?;
        }
        Ok(())
    }
}

impl Argument {
    fn set_name(&mut self, name : &str) {
        self.name = name.trim().to_owned();
    }

    fn set_type(&mut self, type_ : &str) {
        self.type_ = type_.trim().to_owned();
    }

    fn set_default_value(&mut self, default_value : &str) {
        self.default_value = default_value.trim().to_owned();
    }
}

// -----------------------------------------------

pub fn main() {
    match parse() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

pub fn parse() -> Result<(), AppError> {

    let file = File::open(r"E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\docs\doxygen\out\html\classwx_frame.html")?;
    let reader = BufReader::new(file);

    let mut data : bool = false;

    for ol in reader.lines() {
        let mut line = ol?;

        if !data {
            data = line.starts_with("Public Member Functions");
        }
        else {
            // end of data
            if line.starts_with(r#"<h2 class="groupheader">Constructor"#) { break; }

            // skip lines not containing a method description
            if !line.starts_with(r#"<tr class="memitem"#) { continue; }

            line = remove_tags(&line).replace("&#160;", "").replace("&amp;", "&");
            line = line.trim().to_owned();

            if line.contains("wxStatusBar") {
                let n  = 0;
            }

            if let Some(p1) = line.find('(') {
                if let Some(p2) = line.find(')') {

                    let mut method = Method::default();

                    // method name and virtual
                    // extract all upto first bracket and split into tokens
                    let s1 = remove_unwanted_spaces(&line[0..p1]);
                    let s2  = remove_unwanted_spaces(&line[(p1+1)..p2]);
                    let ss : Vec<&str> = s1.split(' ').collect();

                    if ss.len() == 1 {
                        method.is_constructor = true;
                        method.set_name(ss[0]);
                    } else if ss.len() == 2 {
                        if ss[0] == "virtual" && ss[1].starts_with("~"){
                            method.is_virtual = true;
                            method.is_destructor = true;
                            method.set_name(&ss[1]);
                        } else {
                            method.set_return_type(&ss[0]);
                            method.set_name(&ss[1]);
                        }
                    } else {
                        let mut ix = 0;
                        if ss[ix] == "virtual" {
                            method.is_virtual = true;
                            ix += 1;
                        }

                        method.set_return_type(ss[ix]);
                        ix += 1;

                        if ss[ix] == "*" {
                            method.is_pointer = true;
                            ix += 1;
                        }
                        if ss[ix] == "&" {
                            method.is_ref = true;
                            ix += 1;
                        }

                        method.set_name(ss[ix]);
                    }

                    // parse method arguments
                    for a in s2.split(',') {
                        let a = a.replace("=", "= ");
                        let a = remove_unwanted_spaces(&a);
                        if !a.is_empty() {
                            let aps : Vec<&str> = a.split(' ').collect();
                            let mut arg = Argument::default();
                            let mut ix = 0;

                            if aps[ix] == "const" {
                                arg.is_const = true;
                                ix += 1;
                            }

                            arg.set_type(aps[ix]);
                            ix += 1;

                            let mut t = aps[ix];
                            if t.starts_with('&') {
                                arg.is_ref = true;
                                t = &t[1..];
                            }
                            else if t.starts_with('*') {
                                arg.is_pointer = true;
                                t = &t[1..];
                            }

                            if t.ends_with('=') {
                                arg.set_default_value(aps[ix+1]);
                                t = &t[0..t.len()-1];
                            }
                            arg.set_name(t);

                            method.add_argument(arg);
                        }
                    }
                    println!("Method ====================\n{}", &method);
                }
            } 
        }
    }
    Ok(())
}

fn remove_tags(in_str : &str) -> String {
    let mut outs = in_str.to_owned();
    while true {
        if let Some(p1) = outs.find('<') {
            if let Some(p2) = outs.find('>') {
                if p1 < p2 { outs = outs[..p1].to_owned() + " " + &outs[(p2+1usize)..]; }
                else { outs = outs[(p2+1usize)..].to_owned(); }
            }
            else { outs = outs[..p1].to_owned(); }
        }
        else { break };
    }
    outs
}

fn remove_unwanted_spaces(s : &str) -> String {
    let mut s = s.trim().to_owned();
    while let Some(_) = s.find("  ") {
        s = s.replace("  ", " ");
    }
    s
}

