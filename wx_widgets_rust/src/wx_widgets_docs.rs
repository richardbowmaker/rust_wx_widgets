
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

    let file = File::open(r"E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\docs\doxygen\out\html\classwx_frame.html")?;
    let reader = BufReader::new(file);

    let mut data : bool = false;


// --------------------------------

    // regex to extract html tags from a line
    let rex_html = Regex::new(r"(<[^>]+>)([^<]*)")?;

/*

parse a method line such as;

    virtual bool  Create ( wxWindow *parent, wxWindowID id, const wxString &title, const wxPoint &pos= wxDefaultPosition , const wxSize &size= wxDefaultSize , long style= wxDEFAULT_FRAME_STYLE , const wxString &name=wxFrameNameStr) 

(?x)                 # free space mode
\s*(virtual)?        # optional virtual method
\s*(\w*)             # return type 
\s*(\w*)             # method name
\s*\(                # opening bracket

 
(?x)  \s*(virtual)?  \s*(\w*)  \s*(\w*)  \s*\(*

*/

    let rex_method = Regex::new(r"(?x)  \s*(virtual)?  \s*(\w*)  \s*(\w*)  \s*\(")?;

/*

parse method parameters into const, pointer/reference, type, name, default value

(?x) 
\s*(const)?
\s*(\w*)
\s*(\&|\*)?
\s*(\w*)
\s*((=)\s*(\w*))?

(?x)   \s*(const)?  \s*(\w*)  \s*(\&|\*)?  \s*(\w*)  \s*((=)\s*(\w*))?

*/

let rex_parameters = Regex::new(r"(?x)   \s*(const)?  \s*(\w*)  \s*(\&|\*)?  \s*(\w*)  \s*((=)\s*(\w*))?")?;

//--------------------------------------

    for ol in reader.lines() {
        let mut line = ol?;

        if !data {
            data = line.starts_with("Public Member Functions");
        }
        else {
            if line.starts_with(r#"<h2 class="groupheader">Constructor"#) { break; }

            println!("html - {}", &line);

            let mut s = String::from("");

            // --------------
            // extract html tags

            for caps in rex_html.captures_iter(&line) {
                if caps.len() > 2 {
                    s.push_str(&caps[2]);
                    s.push(' ');
                }
                else { continue; }
            }

            let s = s.replace("&#160;", "");
            let s = s.replace("&amp;", "&");
            let line = s;
            let mut s = String::from("");

            println!("text - {}", &line);

            // ----------------------
            // parse method signature, excluding parameters

            let oms = rex_method.captures(&line);

            if let Some(ms) = oms {
                println!("Method ***************************");
                println!("captures {} : ", &ms.len());
                println!("{:?} : ", &ms);
                let x = 0;

                // 1 - virtual, 2 - return type, 3 - name
    
            }

            // -------------------------
            // parse all arguments

            let oms1 = rex_parameters.captures(&line);

            if let Some(ms1) = oms1 {
                println!("Parameters ***************************");
                println!("captures {} : ", &ms1.len());
   
                // for cap in ms1 {
                //     println!("{:?} : ", &ms1);
                //     let x = 0;
                // }

                let n = 0;
            }

            // let parms = rex_parameters.find_iter(&line);


            // for parm in parms {
            //     println!("param {}", parm.as_str());
                
            // }


 


            // let oparms = rex_method.captures_iter(&line).nth(0);

            // if let Some(parms) = oparms {
            //      let oparms = parms.iter().nth(5);

            //      if let Some(Some(ps)) = oparms {
                     
            //          let ss = ps.as_str();

            //          println!("parameters ps {:?}", ps);
            //          println!("parameters ps {}", &ss);
            //          let z = ps;
            //          let x = 0; 
            //      }
            // }

 


            // for cap in rex_method.captures_iter(&line) {

            //     println!("**********************");
            //     println!("cap {} {:?}", n, cap);
            //     let x = 0;
            //     n = n + 1;

            //     // if caps.len() > 0 {
            //     //     for cap in caps.iter() {
            //     //         if let Some(tok) = cap {
            //     //             s.push_str(&tok.as_str());
            //     //             s.push_str(" $$ ");
            //     //         }
            //     //     }
            //     //     println!("tokens : {}", &s);
            //     }

            //     // println!("caps len {}", caps.len());
            //     // if caps.len() == 6 {

            //     //     let x = &caps[4];
            //     //     println!("token[4] {}", &x);
            //     // }
            //     // else { continue; }
            // //}


        }
    }
    Ok(())
}

