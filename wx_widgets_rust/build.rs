fn main() {
    println!("cargo:rustc-link-lib=dylib=wxWidgetsDll2");
    println!(r"cargo:rustc-link-search=native=D:\projects\rust_and_c\wxWidgetsDll2\x64\Debug");
    println!(r"cargo:rustc-link-search=native=D:\projects\rust_and_c\wxWidgetsDll2Client\x64\Debug");
    println!(r"cargo:rustc-link-search=native=E:\_Ricks\c++\wxWidgets\3.1.3\x64\wxMSW-3.1.3_vc14x_x64_ReleaseDLL\lib\vc14x_x64_dll");


// PATH=%PATH%;E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\lib\vc_x64_dll;D:\projects\rust_and_c\wxWidgetsDll2\x64\Debug;D:\projects\rust_and_c\wxWidgetsDll2Client\x64\Debug

}

