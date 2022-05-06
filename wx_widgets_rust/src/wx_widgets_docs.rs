

use std::fs::File;
use std::io::{prelude::*, BufReader};
use crate::errors::AppError;
use std::default::Default;
use std::fmt;



// E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\include\wx\msw

// -----------------------------------------------
// Method type
pub struct Method {
    is_virtual : bool, 
    name : String,
    arguments : Vec<Argument>,
}

impl Default for Method {
    fn default() -> Self {
        Self {is_virtual : false, name : String::from(""), arguments : Vec::new()} 
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_virtual { write!(f, "virtual ")?; }
        write!(f, "{} (", &self.name);
        let mut sep = "";
        for arg in &self.arguments {
            write!(f, "{}{}", &sep, arg);
            sep = ", ";
        }
        write!(f, ")")
    }
}

impl Method {
    fn add_argument(&mut self, arg : Argument) {
        self.arguments.push(arg);
    }
}

// -----------------------------------------------
// Argument type
//
// const wxPoint &pos= wxDefaultPosition

pub struct Argument {
    is_const : bool, 
    is_ref : bool,
    is_pointer : bool,
    type_ : String,
    name : String,
    default_value : String,
}

impl Default for Argument {
    fn default() -> Self {
        Self{is_const : false, is_ref : false, is_pointer : false, type_ : String::from(""), name : String::from(""), default_value : String::from("")} 
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


// -----------------------------------------------

pub fn main() {
    match parse() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

pub fn parse() -> Result<(), AppError> {

    // let s1 = extract_tags("123<456>789");
    // println!("{}", &s1);
    // let s1 = extract_tags("<456>789");
    // println!("{}", &s1);
    // let s1 = extract_tags("123<456>");
    // println!("{}", &s1);
    // let s1 = extract_tags("123<456");
    // println!("{}", &s1);
    // let s1 = extract_tags("123><567");
    // println!("{}", &s1);
    // let s1 = extract_tags("123<456>789<abc>xyz");
    // println!("{}", &s1);
    // let s1 = extract_tags("123<456>789<abc>");
    // println!("{}", &s1);
    // let s1 = extract_tags("123<456>789<abc");
    // println!("{}", &s1);
    // let s1 = extract_tags("<><<>><<>>><<");
    // println!("{}", &s1);


    let file = File::open(r"E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\docs\doxygen\out\html\classwx_frame.html")?;
    let reader = BufReader::new(file);

    let mut data : bool = false;


    for ol in reader.lines() {
        let line = ol?;

        if !data {
            data = line.starts_with("Public Member Functions");
        }
        else {
            if line.starts_with(r#"<h2 class="groupheader">Constructor"#) { break; }

            // println!("html - {}", &line);
            let line = remove_tags(&line).replace("&#160;", "").replace("&amp;", "&");
            println!("text - {}", &line);

            if let Some(p1) = line.find('(') {
                if let Some(p2) = line.find(')') {

                    // extract all upto first bracket and split into tokens
                    let s1 = &line[0..p1]; 
                    let s2  = &line[(p1+1)..p2];

                    let mut m = Method::default();

                    for s in s1.split(' ') {
                        let t = s.trim();
                        if t == "virtual" {m.is_virtual = true; }
                        else { 
                            m.name = String::from(t);
                            break;
                         }
                    }

                    for s in s2.split(',') {
                        let s = s.trim();
                        if !s.is_empty() {
                            let ss : Vec<&str> = s.split(' ').collect();
                            let mut arg = Argument::default();
                            let mut ix = 0;

                            if ss[ix] == "const" {
                                arg.is_const = true;
                                ix += 1;
                            }

                            arg.type_ = String::from(ss[ix]);
                            ix += 1;

                            let mut t = ss[ix];
                            if t.starts_with('&') {
                                arg.is_ref = true;
                                t = &t[1..];
                            }
                            else if t.starts_with('*') {
                                arg.is_pointer = true;
                                t = &t[1..];
                            }

                            let tt : Vec<&str> = t.split('=').collect();
                            arg.name = String::from(tt[0]);
                            if tt.len() > 1 {
                                arg.default_value = String::from(tt[1]);
                            }

                            m.add_argument(arg);
                        }
                    }
                    println!("Method ====================\n{}", &m);
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
                if p1 < p2 { outs = outs[..p1].to_owned() + &outs[(p2+1usize)..]; }
                else { outs = outs[(p2+1usize)..].to_owned(); }
            }
            else { outs = outs[..p1].to_owned(); }
        }
        else { break };
    }
    outs
}

