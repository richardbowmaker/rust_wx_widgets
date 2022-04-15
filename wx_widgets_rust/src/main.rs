

use libloading::*;
use std::process::Command;
use std::env;
use std::io::{self, Write};
use cpp_core::*;
use std::ops::{Deref, DerefMut};
use once_cell::sync::OnceCell;


#[macro_use]
extern crate lazy_static;



// https://crates.io/


mod wx_widgets;
use wx_widgets::*;

mod errors;
use errors::*;



fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let lib = Library::new(r"wxWidgetsDll2.dll")?;
        let func: libloading::Symbol<unsafe extern fn(u64, u64, u64, i32) -> u32> = lib.get(b"start_wxapp")?;
        Ok(func(0, 0 , 0, 0))
    }
}

fn load_library(libname: &str) -> Option<Library> {
    unsafe {
        match Library::new(libname) {
            Ok(lib) => Some(lib),
            Err(err) => {
                println!("error loading library {}, {}", libname, err);
                None
            },
        }
    }
}

type callback = unsafe extern "C" fn(u32) -> u32;

// links to DLL wxWidgetsDll2, see build.rs
extern "C" {
    fn start_wxapp(a: u64, b: u64, c: u64, d: i32) -> u32;
    fn callback_test2(a: u64);
}

fn callback_test2_ex(fn1: callback) {
    unsafe {
        callback_test2(fn1 as u64);
    } 
}

#[derive(Debug)]
struct MyStruct {
    value : u32,
}

impl MyStruct {
    fn set(&mut self, v : u32) {
        self.value = v
    }
}




lazy_static! {
    static ref MY_STRUCT: MyStruct = MyStruct{ value : 22 };
}

// --------------------------------------------------------

static mut mx : u32 = 23;


fn getmx() -> u32 {
    unsafe {
        mx
    }
}

fn setmx(val : u32 ){
    unsafe {
        mx = val
    }
}

// --------------------------------------------------------

static INSTANCE: OnceCell<MyStruct> = OnceCell::new();

impl MyStruct {
    pub fn global() -> &'static MyStruct {
        INSTANCE.get().expect("logger is not initialized")
    }

 
}







fn main() {
    println!("wx widgets");
    println!("global singleton 1 {}", getmx());
    setmx(34);
    println!("global singleton 2 {}", getmx());

    let ms = MyStruct{ value: 56 };
    INSTANCE.set(ms).unwrap();

    println!("global singleton 3 {}", MyStruct::global().value);

    // let ms1 = MyStruct{ value: 78 };
    // INSTANCE.set(ms1).unwrap();
    

    



    // uppdate PATH environment variable
    // let key = "PATH";

    // let mut path = match env::var_os(key) {
    //     Some(val) => {
    //         // println!("{}: {:?}", key, val);
    //         val
    //     }
    //     None => {
    //         println!("{} is not defined in the environment.", key);
    //         return ()
    //     }
    // };

    // path.push(";");
    // path.push(r"E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\lib\vc_x64_dll");
    // path.push(";");
    // path.push(r"D:\projects\rust_and_c\wxWidgetsDll2\x64\Debug");

    // env::set_var(key, path);



    // unsafe {
    //     start_wxapp(0, 0, 0, 0);
    //     // callback_test2_ex(func2);
    // }






    

    // let wx = wx_widgets_lib::new();



// -----------------------------------------------    

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

//     env::set_var(key, path);

//  // ------------------------------------------------

//     let lib = load_library(r"wxWidgetsDll2.dll").expect("");


//     unsafe {
//         let func: libloading::Symbol<unsafe extern fn(u64, u64, u64, i32) -> u32> = lib.get(b"start_wxapp").expect("error loading start_wxapp function");
//         func(0, 0 , 0, 0);
//     }

    // unsafe {
    //     let func: libloading::Symbol<unsafe extern fn(u64) -> u32> = lib.get(b"callback_test").expect("error loading callback_test function");
    //     let ptr = func1 as u64;
    //     func(ptr);
    // }

    // unsafe {
    //     type callback = unsafe extern "C" fn(u32) -> u32;
    //     let func: libloading::Symbol<unsafe extern fn(callback)> = lib.get(b"callback_test2").expect("error loading callback_test2 function");
    //     // let ptr = func2 as u64;
    //     func(func2);
    // }

    // callback: unsafe extern "C" fn(*mut u8, usize)

    
//     let output = Command::new("cmd")
//                         .arg(r"/c")
//                         .arg(r"D:\projects\rust_and_c\wx_widgets_rust\wxWidgets.bat")
//  //                       .env("PATH",path)
//                         .output()
//                         .expect("failed to execute process");

//     println!("status: {}", output.status);
//     io::stdout().write_all(&output.stdout).unwrap();
//     io::stderr().write_all(&output.stderr).unwrap();

//     assert!(output.status.success());

//     // Command::new("cmd")
//     //         .output()
//     //         .expect("failed to execute process");


    // match call_dynamic() {
    //     Ok(_) => {
    //         println!("library loaded OK");
    //         return ()
    //     },
    //     Err(_) => {
    //         println!("Failed to load library");
    //         return ()
    //     },
    // }

// --------------------------------------------------------------

 //       let f = go_through_pointer(func1);


        return ()
}

#[no_mangle]
pub extern "C" fn rust_function() {
    println!("Called from C")
}

fn go_through_pointer(x: fn()) -> fn() {
    let ptr = x as *const ();
    unsafe { std::mem::transmute::<*const (), fn()>(ptr) }
  }

  unsafe extern "C"  fn func1() {
      println!("In func1");
  }

unsafe extern "C"  fn func2(n: u32) -> u32 {
    println!("In func2");
    n * 2
}



