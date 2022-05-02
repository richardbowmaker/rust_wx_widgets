
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


    // regex to extract html tags from a line
    let rex_html = Regex::new(r"(<[^>]+>)([^<]*)")?;

/*

parse a method line such as;

    virtual bool  Create ( wxWindow *parent, wxWindowID id, const wxString &title, const wxPoint &pos= wxDefaultPosition , const wxSize &size= wxDefaultSize , long style= wxDEFAULT_FRAME_STYLE , const wxString &name=wxFrameNameStr) 

(?x)                 # free space mode
\s*(virtual)?        # optional virtual method
\s*(\w*)             # return type 
\s*(\w*)             # method name
\s*\(\s*             # opening bracket
(([^,\)]+)[,\)])*    # repeated parameters



(?x)  \s*(virtual)?  \s*(\w*)  \s*(\w*)  \s*\(\s*  (([^,\)]+)[,\)])*    

*/

    let rex_method = Regex::new(r"(?x)  \s*(virtual)?  \s*(\w*)  \s*(\w*)  \s*\(\s*  (([^,\)]+)[,\)])*")?;

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
            let line = s.clone();
            let mut s = String::from("");

            println!("text - {}", &line);

            // ----------------------
            // parse method signature

            for caps in rex_method.captures_iter(&line) {

                if caps.len() > 0 {

                    for cap in caps.iter() {

                        if let Some(tok) = cap {
                            s.push_str(&tok.as_str());
                            s.push_str(" $$ ");
                        }
                    }
                    println!("{}", &s);
                }
                else { continue; }
            }
        }
    }

    Ok(())



    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$")?;
    // let x = re.is_match("2014-01-01");


    // E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\docs\doxygen\out\html\classwx_frame.html

//    let re = Regex::new(r"(.*?)(<a\b[^>]*>(.*?)</a>)*")?;
//    let re = Regex::new(r"(<[^>]+>)([^<]*)")?;
//    let text = r#"<tr class="memitem:af80368ba23c71c5d947c3178b8fe10fc"><td class="memItemLeft" align="right" valign="top">&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc">wxFrame</a> ()</td></tr>"#;
//    let text = r#"<tr class="memitem:af80368ba23c71c5d947c3178b8fe10fc"><a class="el" href="classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc">wxFrame</a> ()</td></tr>"#;
//    let text = r#"<tr class="memitem:a01b53ac2d4a5e6b0773ecbcf7b5f6af8"><td class="memItemLeft" align="right" valign="top">&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="classwx_frame.html#a01b53ac2d4a5e6b0773ecbcf7b5f6af8">wxFrame</a> (<a class="el" href="classwx_window.html">wxWindow</a> *parent, <a class="el" href="windowid_8h.html#ae8091432cc2cb2485d45f2302fb51133">wxWindowID</a> id, const <a class="el" href="classwx_string.html">wxString</a> &amp;title, const <a class="el" href="classwx_point.html">wxPoint</a> &amp;pos=<a class="el" href="gdicmn_8h.html#af5a90c753eaf3d3e3e5068a19ec3c1d0">wxDefaultPosition</a>, const <a class="el" href="classwx_size.html">wxSize</a> &amp;size=<a class="el" href="gdicmn_8h.html#a33a012cdb075e9f78c93f63bec2dc27b">wxDefaultSize</a>, long style=<a class="el" href="toplevel_8h.html#ab3ad2777e5178344fa798ec8fd8e95e2">wxDEFAULT_FRAME_STYLE</a>, const <a class="el" href="classwx_string.html">wxString</a> &amp;name=wxFrameNameStr)</td></tr>"#;    

    // if re.is_match(text) {
    //     println!("Regex matched OK");
    // }
    // else {
    //     println!("*** Regex match failed ***");
    // }
    
    // let m = re.find(text).ok_or(AppError::new("Regex match"))?;
    // println!("Regex find ({}:{}) - {}", m.start(), m.end(), m.as_str());
    // let mut s = String::from("");

    // for caps in re.captures_iter(text) {
    //     println!("capture : ");
    //     if caps.len() > 0 {
    //         for cap in caps.iter() {
    //             if let Some(m) = cap {
    //                 println!("match : {}", &m.as_str());
    //             }
    //         }
    //         s.push_str(&caps[2]);
    //     }
    // }

    // let s = s.replace("&#160;", "");
    // let s = s.replace("&amp;", "&");

    // println!("finally : {}", &s);

    // let om = re.find("2014-01-01");

    // if let Some(m) = om {
    //     let s = m.as_str();
    //     let y = 0;
    // }

    // <tr class="memitem:af80368ba23c71c5d947c3178b8fe10fc"><td class="memItemLeft" align="right" valign="top">&#160;
    //      </td><td class="memItemRight" valign="bottom"><a class="el" href="classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc">wxFrame</a> ()</td></tr>

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
