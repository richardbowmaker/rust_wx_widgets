
// https://crates.io/



mod wx_widgets_if;
mod errors;
mod utilities;
mod main_frame;
mod wx_code_generator;
mod wx_widgets_docs_parser;

fn main() {
    println!("wx widgets");
    let mut data = wx_code_generator::WxCodeData::new();

    match wx_widgets_docs_parser::parse(&mut data) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    print!("Data\n{}", &data);
 
    return ()
}








