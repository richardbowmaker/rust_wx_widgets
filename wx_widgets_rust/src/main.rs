
// https://crates.io/

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops:: {Deref, DerefMut};
use std::cell::Ref;


mod wx_widgets;
mod errors;
mod utilities;
mod main_frame;
mod code_generator;
mod code_generator1;
mod wx_widgets_docs;
mod wx_widgets_source_parser;



fn main() {
    println!("wx widgets");
    // code_generator1::create("");
    wx_widgets_docs::main();



 
    return ()
}








