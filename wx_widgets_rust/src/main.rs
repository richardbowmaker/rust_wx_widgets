
// https://crates.io/



mod wx_widgets_if;
mod errors;
mod utilities;
mod main_frame;
mod wx_code_generator;
mod wx_widgets_docs_parser;

fn f1(t : &(i32, i32)) -> i32 {
    t.0 + t.1
}

fn main() {
    println!("wx widgets");

    let x = (1, 2);
    let s = f1(&x);

    let y = x.0;
    let z = x.1;
    let n = 0;

    main_frame::main();


    // match run() {
    //     Ok(_) => {},
    //     Err(e) => {
    //         println!("{}", e);
    //     },
    // }
}

fn run() -> Result<(), errors::AppError> {
    let config_filename = r"D:\projects\rust_and_c\config.json";
    
    let mut data = wx_code_generator::WxCodeData::default();
    wx_widgets_docs_parser::parse(&mut data)?;
    println!("{}", data);
    let mut rust_code = wx_code_generator::Code::new();
    wx_code_generator::generate_rust_code(&data, &mut rust_code)?;
    wx_code_generator::update_rust_templates(&data, &rust_code, "wxFrame.rs")?;
    Ok(())
}
