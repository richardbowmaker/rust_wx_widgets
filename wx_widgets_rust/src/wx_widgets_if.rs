
use std::ffi::CString;
use std::os::raw::{c_char, c_void, c_int};
use once_cell::sync::OnceCell;

use crate::errors::AppError;

// string from C to rust
// https://stackoverflow.com/questions/50437953/provide-char-argument-to-c-function-from-rust



pub const WX_ID_EXIT : i32 = 5006;
pub const WX_ID_ABOUT : i32 = 5014;

type OnInitEventT = fn();
type OnInitEventExternT = unsafe extern "C" fn(on_init : *const c_void);
type OnMenuEventExternT = unsafe extern "C" fn(wx_frame : *const c_void, wx_command_event : *const c_void, wx_handler : *const c_void) -> *const c_void;

extern "C" {
    fn init_wx_widgets_extern(hInstance : u64, hPrevious : u64, pCmdLine : *const c_char, nCmdShow : i32, on_init_extern : *const c_void, on_init : *const c_void) -> *const c_void;
    
    fn wx_create_frame_extern(text : *const c_char, point_x : i32, point_y : i32 , size_w : i32 , size_h : i32) -> *const c_void;
    fn wx_frame_show_extern(wx_frame : *const c_void, show : i32) -> i32;
    fn wx_create_menu_extern() -> *const c_void;
    fn wx_menu_append_extern(wx_menu : *const c_void, wx_menu_id : i32) -> *const c_void;
    fn wx_create_menu_bar_extern() -> *const c_void;
    fn wx_menu_bar_append_extern(wx_menu_bar : *const c_void, wx_menu : *const c_void, text : *const c_char) -> i32;
    fn wx_frame_set_menu_bar_extern(wx_frame : *const c_void, wx_menu_bar : *const c_void);
    fn wx_frame_close_extern(wx_frame : *const c_void);

    fn wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED_extern(wx_frame : *const c_void, wx_menu : *const c_void, wx_on_menu_handler : *const c_void, wx_menu_id : i32, handler : *const c_void);

    // new

    fn wx_frame_create_extern_1(parent : *const c_void, id : i32, title : *const c_char) -> *const c_void;
    fn wx_frame_create_extern_2(parent : *const c_void, id : i32, title : *const c_char, point_x : i32, point_y : i32) -> *const c_void;
    fn wx_frame_create_extern_3(parent : *const c_void, id : i32, title : *const c_char, point_x : i32, point_y : i32, size_w : i32 , size_h : i32) -> *const c_void;
    fn wx_frame_create_extern_4(parent : *const c_void, id : i32, title : *const c_char, point_x : i32, point_y : i32, size_w : i32 , size_h : i32, style : i32) -> *const c_void;
    fn wx_frame_create_extern_5(parent : *const c_void, id : i32, title : *const c_char, point_x : i32, point_y : i32, size_w : i32 , size_h : i32, style : i32, name : *const c_char) -> *const c_void;

}



// ---------------------------------------------------------------------
// WxFrame

//static INSTANCE: OnceCell<WxFrame> = OnceCell::new();

type OnMenuEventHandlerT = fn(frame : &WxFrame, event : &WxCommandEvent);

macro_rules! to_cstr{
       ($str:expr)=>{
               CString::new($str)?.as_ptr()
       }
    }

pub struct WxWindow {
    window : *const c_void, 
}

#[derive(Debug)]
pub struct WxFrame{
    main_frame : *const c_void,
}

// impl WxFrame {
//     pub fn instance() -> &'static WxFrame {
//         INSTANCE.get().expect("WxFrame is not initialized")
//     }
// }

impl WxFrame {

    pub fn new(frame : *const c_void) -> Self {
        WxFrame { main_frame : frame }
    }

    pub fn create(parent : Option<&WxWindow>, id : i32, title : &str, pos : Option<(i32, i32)>, size : Option<(i32, i32)>, style : Option<i32>, name : Option<&str> ) -> Result<Self, AppError> {
        unsafe {
            let mut frame   : *const c_void = 0 as *const c_void;
            let mut parent_ : *const c_void = 0 as *const c_void;

            if let Some(p) = parent {
                parent_ = p.window;
            }

            if let Some((x, y)) = pos {
                if let Some((w, h)) = size {
                    if let Some(style_) = style {
                        if let Some(name_) = name {
                            frame = wx_frame_create_extern_5(parent_, id, to_cstr!(title), x, y, w, h, style_, to_cstr!(name_));
                        }
                        else {
                            frame = wx_frame_create_extern_4(parent_, id, to_cstr!(title), x, y, w, h, style_);
                        }
                    }
                    else {
                        frame = wx_frame_create_extern_3(parent_, id, to_cstr!(title), x, y, w, h);
                    }
                }
                else {
                    frame = wx_frame_create_extern_2(parent_, id, to_cstr!(title), x, y);
                }
            }
            else {
                frame = wx_frame_create_extern_1(parent_, id, to_cstr!(title));
            }
            Ok(WxFrame::new(frame))
        }
    }

    pub fn show(&self, show : bool) {
        unsafe {
            wx_frame_show_extern(self.main_frame, show as i32);
        }
    }

    pub fn close(&self) {
        unsafe {
            wx_frame_close_extern(self.main_frame);
        }
    }

    pub fn set_menu_bar(&self, menu_bar : &WxMenuBar) {
        unsafe {
            wx_frame_set_menu_bar_extern(self.main_frame, menu_bar.raw());
        }
    }

    pub fn bind_menu_event_handler(&self, menu_item : &WxMenuItem, wx_menu_id: i32, handler : OnMenuEventHandlerT) {
        unsafe {
            wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED_extern(
                self.main_frame, 
                menu_item.raw(), 
                WxFrame::on_menu_event_handler_extern as *const c_void, 
                wx_menu_id,
                handler as *const c_void);
        }
    }

    unsafe extern "C" fn on_menu_event_handler_extern(frame : *const c_void, event : *const c_void, wx_handler : *const c_void) {
        let f = WxFrame::new(frame);
        let h : OnMenuEventHandlerT = std::mem::transmute(wx_handler);
        let e = WxCommandEvent::new(event);
        h(&f, &e);
    }
}

// --------------------------------------------------------------------
// WxMenu

#[derive(Debug)]
pub struct WxMenu{
    menu : *const c_void,
}

impl WxMenu {

    pub fn new() -> Self {
        unsafe {
            WxMenu { menu : wx_create_menu_extern() }
        }
    }

    pub fn raw(&self) -> *const c_void {
        self.menu
    }

    pub fn append(&self, menu_id : i32) -> WxMenuItem {
        unsafe {
            let mi = wx_menu_append_extern(self.menu, menu_id);
            WxMenuItem::new(mi)
        }
    }
}

// --------------------------------------------------------------------
// WxMenuItem

#[derive(Debug)]
pub struct WxMenuItem{
    menu_item : *const c_void,
}

impl WxMenuItem {

    pub fn new( wx_menu_item : *const c_void) -> Self {
        WxMenuItem { menu_item : wx_menu_item }
    }

    pub fn raw(&self) -> *const c_void {
        self.menu_item
    }

    pub fn append(&self, menu_id : i32) {
        unsafe {
            wx_menu_append_extern(self.menu_item, menu_id);
        }
    }
}

// --------------------------------------------------------------------
// WxMenuBar

#[derive(Debug)]
pub struct WxMenuBar{
    menu_bar : *const c_void,
}

impl WxMenuBar {

    pub fn new() -> Self {
        unsafe {
            WxMenuBar { menu_bar : wx_create_menu_bar_extern() }
        }
    }

    pub fn raw(&self) -> *const c_void {
        self.menu_bar
    }

    pub fn append(&self, menu : &WxMenu, text : &str) {
        unsafe {
            let s = CString::new(text).expect("CString::new failed");
            wx_menu_bar_append_extern(self.menu_bar, menu.raw(), s.as_ptr());
        }
    }
}

// --------------------------------------------------------------------
// WxCommandEvent

pub struct WxCommandEvent {
    event : *const c_void,
}

impl WxCommandEvent {
    pub fn new(e : *const c_void) -> Self {
        WxCommandEvent { event : e }
    }
}

// --------------------------------------------------------------------

unsafe extern "C" fn on_init_extern(on_init : *const c_void) {
    let f : OnInitEventT = std::mem::transmute(on_init);
    f();
}

pub fn initialise(on_init : OnInitEventT) -> Result<(), AppError>
{
    unsafe {
        init_wx_widgets_extern(
            0, 
            0, 
            to_cstr!(""), 
            0, 
            on_init_extern as *const c_void, 
            on_init as  *const c_void);
    }
    Ok(())
}



