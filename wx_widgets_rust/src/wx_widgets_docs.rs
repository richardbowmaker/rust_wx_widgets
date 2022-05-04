
use html_parser::Dom;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;
use crate::errors::AppError;



// E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\include\wx\msw


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

            println!("html - {}", &line);
            let line = remove_tags(&line).replace("&#160;", "").replace("&amp;", "&");
            println!("text - {}", &line);

            for token in line.split(" ") {
                let token = token.trim();
                println!("token {}", &token);
            }

            let p = 0;
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

