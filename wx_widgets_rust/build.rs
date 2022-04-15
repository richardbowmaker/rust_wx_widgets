
use std::env;



fn main() {

    println!("cargo:rustc-link-lib=dylib=wxWidgetsDll2");
    println!(r"cargo:rustc-link-search=native=D:\projects\rust_and_c\wxWidgetsDll2\x64\Debug");
    println!(r"cargo:rustc-link-search=native=D:\projects\rust_and_c\wxWidgetsDll2Client\x64\Debug");
    println!(r"cargo:rustc-link-search=native=E:\_Ricks\c++\wxWidgets\3.1.3\x64\wxMSW-3.1.3_vc14x_x64_ReleaseDLL\lib\vc14x_x64_dll");

    

//     // uppdate PATH environment variable
//     let key = "PATH";

//     let mut path = match env::var_os(key) {
//         Some(val) => {
//             // println!("{}: {:?}", key, val);
//             val
//         }
//         None => {
//             println!("{} is not defined in the environment.", key);
//             return ()
//         }
//     };

//     path.push(";");
//     path.push(r"E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\lib\vc_x64_dll");
//     path.push(";");
//     path.push(r"D:\projects\rust_and_c\wxWidgetsDll2\x64\Debug");

//     // env::set_var(key, path);




//     println!(r"cargo:rustc-env=PATH={:?}", path);
//     // println!(r"cargo:rustc-env=PATH=E:\_Ricks\c++\wxWidgets\3.1.3\x64\wxMSW-3.1.3_vc14x_x64_ReleaseDLL\lib\vc14x_x64_dll");
}

