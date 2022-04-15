
use std::ffi::CString;
use std::os::raw::c_char;
use utf16_lit::utf16_null;

// string from C to rust
// https://stackoverflow.com/questions/50437953/provide-char-argument-to-c-function-from-rust


type wx_object_ptr_t = u64;
type wx_bool = u64;
type wx_void = u64;

const wxID_EXIT_ : u64 = 5006;
const wxID_ABOUT_ : u64 = 5014;

type on_init_event = unsafe extern "C" fn() -> wx_object_ptr_t;
type on_menu_event = unsafe extern "C" fn(wx_command_event : wx_object_ptr_t) -> wx_bool;

extern "C" {

    fn init_wx_widgets(hInstance : u64, hPrevious : u64, pCmdLine : u64, nCmdShow : i32,  on_init_handler : on_init_event) -> wx_object_ptr_t;

    fn wx_create_frame(text : *const c_char, point_x : u32, point_y : u32 , size_w : u32 , size_h : u32 ) -> wx_object_ptr_t;
    fn wx_frame_show(wx_frame : wx_object_ptr_t, show : wx_bool) -> wx_bool;
    fn wx_create_menu() -> wx_object_ptr_t;
    fn wx_menu_append(wx_menu : wx_object_ptr_t, wx_menu_id : u64) -> wx_object_ptr_t;
    fn wx_create_menu_bar() -> wx_object_ptr_t;
    fn wx_menu_bar_append(wx_menu_bar : wx_object_ptr_t, wx_menu : wx_object_ptr_t, text : *const c_char) -> wx_void;
    fn wx_frame_set_menu_bar(wx_frame : wx_object_ptr_t, wx_menu_bar : wx_object_ptr_t) -> wx_void;
    fn wx_frame_close(wx_frame : wx_object_ptr_t) -> wx_void;
    fn wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED(wx_frame : wx_object_ptr_t, wx_menu : wx_object_ptr_t, wx_on_menu_handler : on_menu_event, wx_menu_id : u64) -> wx_void;
}


unsafe extern "C" fn on_init() -> wx_object_ptr_t {
    unsafe {

        let s1 = CString::new("Hello from rust").expect("CString::new failed");
        // let s2 = CString::new("&File ...").expect("CString::new failed");

        let frame = wx_create_frame(s1.as_ptr(), 50, 50, 450, 340);

        wx_frame_show(frame, 1);
        // let menuFile = wx_create_menu();
        // let menuFileExit = wx_menu_append(menuFile, wxID_EXIT_);
        // wx_menu_append(menuFile, wxID_ABOUT_);
        // let menuBar = wx_create_menu_bar();
        // wx_menu_bar_append(menuBar, menuFile, s2.as_ptr());
        // wx_frame_set_menu_bar(frame, menuBar);
        // wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED(frame, menuFileExit, on_menu_handler, wxID_EXIT_);

        frame
    }
}

unsafe extern "C" fn on_menu_handler(wx_command_event : wx_object_ptr_t) -> wx_bool {
    // wx_frame_close();
    0
}

pub fn wx_widgets_main() {

    unsafe {
        let frame = init_wx_widgets(0, 0, 0, 0, on_init);
        let wf = WxFrame::new(frame);
        wf.init();
    }
}


pub struct WxFrame{
    frame : wx_object_ptr_t,
}

impl WxFrame {

    // fn wx_create_frame(text : *const c_char, point_x : u32, point_y : u32 , size_w : u32 , size_h : u32 ) -> wx_object_ptr_t;
    // let frame = wx_create_frame(s1.as_ptr(), 50, 50, 450, 340);

    pub fn new(f : wx_object_ptr_t) -> Self {
        WxFrame { frame : f }
    }

    pub fn init(&self) {

        unsafe {
            let s2 = CString::new("&File ...").expect("CString::new failed");
            wx_frame_show(self.frame, 1);
            let menuFile = wx_create_menu();
            let menuFileExit = wx_menu_append(menuFile, wxID_EXIT_);
            wx_menu_append(menuFile, wxID_ABOUT_);
            let menuBar = wx_create_menu_bar();
            wx_menu_bar_append(menuBar, menuFile, s2.as_ptr());
            wx_frame_set_menu_bar(self.frame, menuBar);
            wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED(self.frame, menuFileExit, on_menu_handler, wxID_EXIT_);
        }

    }

}



// use libloading::*;
// use std::process::Command;
// use std::env;
// use std::io::{self, Write};
// use cpp_core::*;

// https://doc.rust-lang.org/nomicon/ffi.html

// pub struct wx_widgets_lib<'a> {
//     // dll: libloading::Library,
//     func: libloading::Symbol<'a, unsafe extern fn(u64, u64, u64, i32) -> u32>,
// }

// pub struct wx_frame {
//     frame: Ptr<u64>,

//     // frame: Ptr::from_raw(0 as *const u64),
// }

// impl<'a> wx_widgets_lib<'a> {

    // pub fn new() -> Option<Self> {

    //     // uppdate PATH environment variable to include
    //     // the wx widgets libraries and wx widgets wrapper dll
    //     let key = "PATH";

    //     let mut path = match env::var_os(key) {
    //         Some(val) => {
    //             val
    //         }
    //         None => {
    //             println!("{} is not defined in the environment.", key);
    //             return None
    //         }
    //     };
    
    //     path.push(";");
    //     path.push(r"E:\_Ricks\c++\wxWidgets\3.1.3\wxWidgets-3.1.3\lib\vc_x64_dll");
    //     path.push(";");
    //     path.push(r"D:\projects\rust_and_c\wxWidgetsDll2\x64\Debug");
    
    //     env::set_var(key, path);
       
    //     // load the library

    //     let libname = String::from(r"wxWidgetsDll2.dll");
    //     unsafe {
    //         match Library::new(libname) {
    //             Ok(lib) => {
    //                 match lib.get::<'a, unsafe extern fn(u64, u64, u64, i32) -> u32>(b"start_wxapp") {
    //                     Ok(fn1) => Some (Self { func: fn1 } ),
    //                     Err(_) => None,
    //                 }
    //             }
    //             Err(err) => {
    //                 None
    //             }
    //         }
    //     };

    //     // unsafe {
    //     //     // let func: libloading::Symbol<unsafe extern fn(u64, u64, u64, i32) -> u32> = lib.get(b"start_wxapp").unwrap();
    //     //     // func(0, 0 , 0, 0);

    //     //     // let func: libloading::Symbol<unsafe extern fn(u64, u64, u64, i32) -> u32> = 
    //     //     match lib.get::<unsafe extern fn(u64, u64, u64, i32) -> u32>(b"start_wxapp") {
    //     //         Ok(s) => return None,
    //     //         Err(_) => return None,
    //     //     }

    //     // }
    


    //     None

    // }
// }



    


