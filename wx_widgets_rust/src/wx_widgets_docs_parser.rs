

use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::errors::AppError;
use crate::wx_code_generator;



// -----------------------------------------------

pub fn parse(data : &mut wx_code_generator::WxCodeData) -> Result<(), AppError> {

    let file = File::open(r"E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\docs\doxygen\out\html\classwx_frame.html")?;
    let reader = BufReader::new(file);

    let mut parsing : bool = false;
    let mut class = wx_code_generator::Class::new("WxFrame");

    for ol in reader.lines() {
        let mut line = ol?;

        if !parsing {
            parsing = line.starts_with("Public Member Functions");
        }
        else {
            // end of data
            // if line.starts_with(r#"<h2 class="groupheader">Constructor"#) { break; }
            if line.contains("Public Member Functions inherited from") { break; }

            // skip lines not containing a method description
            if !line.starts_with(r#"<tr class="memitem"#) { continue; }

            line = remove_tags(&line).replace("&#160;", "").replace("&amp;", "&");
            line = line.trim().to_owned();

            if line.contains("wxStatusBar") {
                let n  = 0;
            }

            if let Some(p1) = line.find('(') {
                if let Some(p2) = line.find(')') {

                    let mut method = wx_code_generator::Method::default();

                    // method name and virtual
                    // extract all upto first bracket and split into tokens
                    let s1 = remove_unwanted_spaces(&line[0..p1]);
                    let s2  = remove_unwanted_spaces(&line[(p1+1)..p2]);
                    let ss : Vec<&str> = s1.split(' ').collect();

                    if ss.len() == 1 {
                        method.set_is_constructor(true);
                        method.set_name(ss[0]);
                    } else if ss.len() == 2 {
                        if ss[0] == "virtual" && ss[1].starts_with("~"){
                            method.set_is_virtual(true);
                            method.set_is_destructor(true);
                            method.set_name(&ss[1]);
                        } else {
                            method.set_return_type(&ss[0]);
                            method.set_name(&ss[1]);
                        }
                    } else {
                        let mut ix = 0;
                        if ss[ix] == "virtual" {
                            method.set_is_virtual(true);
                            ix += 1;
                        }

                        method.set_return_type(ss[ix]);
                        ix += 1;

                        if ss[ix] == "*" {
                            method.set_is_pointer(true);
                            ix += 1;
                        }
                        if ss[ix] == "&" {
                            method.set_is_ref(true);
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
                            let mut arg = wx_code_generator::Argument::default();
                            let mut ix = 0;

                            if aps[ix] == "const" {
                                arg.set_is_const(true);
                                ix += 1;
                            }

                            arg.set_type(aps[ix]);
                            ix += 1;

                            let mut t = aps[ix];
                            if t.starts_with('&') {
                                arg.set_is_ref(true);
                                t = &t[1..];
                            }
                            else if t.starts_with('*') {
                                arg.set_is_pointer(true);
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
                    //println!("Method ====================\n{}", &method);
                    class.add_method(method);
                }
            } 
        }
    }
    data.add_class(class);
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

