
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

    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$")?;
    // let x = re.is_match("2014-01-01");


//    let re = Regex::new(r"(.*?)(<a\b[^>]*>(.*?)</a>)*")?;
    let re = Regex::new(r".*?<a\b[^>]*>.*?</a>*")?;
//    let text = r#"<tr class="memitem:af80368ba23c71c5d947c3178b8fe10fc"><td class="memItemLeft" align="right" valign="top">&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc">wxFrame</a> ()</td></tr>"#;
    let text = r#"<tr class="memitem:af80368ba23c71c5d947c3178b8fe10fc"><a class="el" href="classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc">wxFrame</a> ()</td></tr>"#;
    
    if re.is_match(text) {
        println!("Regex matched OK");
    }
    else {
        println!("*** Regex match failed ***");
    }
    
    let m = re.find(text).ok_or(AppError::new("Regex match"))?;
    println!("Regex find ({}:{}) - {}", m.start(), m.end(), m.as_str());

    for caps in re.captures_iter(text) {
        // println!("Movie: {:?}, Released: {:?}",
        //          &caps["title"], &caps["year"]);

        if caps.len() > 0 {
            for cap in caps.iter() {
                if let Some(m) = cap {
                    println!("match : {}", &m.as_str());
                }
            }
        }
    }







    // let om = re.find("2014-01-01");

    // if let Some(m) = om {
    //     let s = m.as_str();
    //     let y = 0;
    // }




    // <tr class="memitem:af80368ba23c71c5d947c3178b8fe10fc"><td class="memItemLeft" align="right" valign="top">&#160;
    //      </td><td class="memItemRight" valign="bottom"><a class="el" href="classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc">wxFrame</a> ()</td></tr>



    let file = File::open(r"D:\projects\rust_and_c\wxwidgets docs\classwx_frame.html")?;
    let reader = BufReader::new(file);

    let mut data : bool = false;

    for ol in reader.lines() {
        let line = ol?;
        if !data {
            data = line.starts_with("Public Member Functions");
        }
        else {


        }
    }

    Ok(())
}

// pub fn parse() {

//     let doc = fs::read_to_string(r"D:\projects\rust_and_c\wxwidgets docs\classwx_frame.html")
//     .expect("Something went wrong reading the file");

//     match Dom::parse(&doc) {
//         Ok(dom) => {
//             println!("Parsed DOM OK");

//             for node in &dom.children {
//                 find_div(node);
//             }

//         }
//         Err(_) => {
//             println!("Error parsing DOM");
//         }
//     }
// }

fn find_div(node : &html_parser::Node) {

    if let Some(e) = node.element() {

        if e.name.eq("div") {

            if let Some(text) = node.text() {
                println!("text -> {}", text);
            }
            println!("attributes {}", e.attributes.len());

            for attr in &e.attributes {

                let &k = &attr.0;

                if let Some(v) = attr.1 {
                    println!("{} -> {}", k, v);
                }

            }
        }
        // }
            // if let Some(oval) = e.attributes.get("class") {
            //     if let Some(val) = oval {
            //         if val == "memproto" {
            //             let y = 0;
            //         }
            //     }
            // }
        for subnode in &e.children {
            find_div(subnode);
        }
    }
}
