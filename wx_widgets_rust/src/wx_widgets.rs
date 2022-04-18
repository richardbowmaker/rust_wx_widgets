
use std::ffi::CString;
use std::os::raw::c_char;
use once_cell::sync::OnceCell;


use crate::utilities;




// string from C to rust
// https://stackoverflow.com/questions/50437953/provide-char-argument-to-c-function-from-rust


type WxObjectPtr = u64;
type WxBool = u64;
type WxVoid = u64;

const WX_ID_EXIT : u64 = 5006;
const WX_ID_ABOUT : u64 = 5014;

type OnInitEvent = unsafe extern "C" fn();
type OnMenuEvent = unsafe extern "C" fn(wx_frame : WxObjectPtr, wx_command_event : WxObjectPtr) -> WxVoid;

extern "C" {
    fn init_wx_widgets(hInstance : u64, hPrevious : u64, pCmdLine : u64, nCmdShow : i32,  on_init_handler : OnInitEvent) -> WxObjectPtr;
    fn wx_create_frame(text : *const c_char, point_x : u32, point_y : u32 , size_w : u32 , size_h : u32 ) -> WxObjectPtr;
    fn wx_frame_show(wx_frame : WxObjectPtr, show : WxBool) -> WxBool;
    fn wx_create_menu() -> WxObjectPtr;
    fn wx_menu_append(wx_menu : WxObjectPtr, wx_menu_id : u64) -> WxObjectPtr;
    fn wx_create_menu_bar() -> WxObjectPtr;
    fn wx_menu_bar_append(wx_menu_bar : WxObjectPtr, wx_menu : WxObjectPtr, text : *const c_char) -> WxVoid;
    fn wx_frame_set_menu_bar(wx_frame : WxObjectPtr, wx_menu_bar : WxObjectPtr) -> WxVoid;
    fn wx_frame_close(wx_frame : WxObjectPtr) -> WxVoid;
    fn wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED(wx_frame : WxObjectPtr, wx_menu : WxObjectPtr, wx_on_menu_handler : OnMenuEvent, wx_menu_id : u64) -> WxVoid;
}


type OnMenuEventHandler = fn(WxCommandEvent : WxObjectPtr) -> WxBool;


// ---------------------------------------------------------------------
// WxFrame

#[derive(Debug)]
pub struct WxFrame{
    main_frame : WxObjectPtr,
}

static INSTANCE: OnceCell<WxFrame> = OnceCell::new();

impl WxFrame {
    pub fn instance() -> &'static WxFrame {
        INSTANCE.get().expect("WxFrame is not initialized")
    }
}

impl WxFrame {

    pub fn new(frame : WxObjectPtr) -> Self {
        WxFrame { main_frame : frame }
    }

    pub fn init(&self) {
        unsafe {
            self.show(true);
            let menu_file = WxMenu::new();
            menu_file.append(WX_ID_ABOUT);
            let menu_file_exit = menu_file.append(WX_ID_EXIT);
            let menu_bar = WxMenuBar::new();
            menu_bar.append(&menu_file, "&File xxx");
            self.set_menu_bar(&menu_bar);
            self.bind_menu_event_handler(&menu_file_exit, WX_ID_EXIT);
        }
    }

    pub fn show(&self, show : bool) {
        unsafe {
            wx_frame_show(self.main_frame, show as WxBool);
        }
    }

    pub fn close(&self) {
        unsafe {
            wx_frame_close(self.main_frame);
        }
    }

    pub fn set_menu_bar(&self, menu_bar : &WxMenuBar) {
        unsafe {
            wx_frame_set_menu_bar(self.main_frame, menu_bar.raw());
        }
    }

    // type OnMenuEvent = unsafe extern "C" fn(wx_command_event : WxObjectPtr) -> WxVoid;
    unsafe extern "C" fn on_menu_1(frame : u64, event : u64) -> u64 {
            let f = WxFrame::new(frame);
            f.close();

        // unsafe {
        //     let a = frame as usize;
        //     let f : &WxFrame = utilities::from_addr(a);
        //     let t = utilities::to_addr(&f);
        //     f.close();
        // }
        0
    }

    pub fn bind_menu_event_handler(&self, menu_item : &WxMenuItem, wx_menu_id: u64) {
        unsafe {


            // {
                
            //     let a1  = utilities::to_addr(&*self);
            //     let wf2: &WxFrame = utilities::from_addr(a1);
            //     let a2  = utilities::to_addr(&wf2);

            //     let t = 0;
            // }


            // let wxf = utilities::to_addr(&*self) as u64;

            wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED(self.main_frame, menu_item.raw(), WxFrame::on_menu_1, wx_menu_id);
        }
    }
}

// --------------------------------------------------------------------
// WxMenu

#[derive(Debug)]
pub struct WxMenu{
    menu : WxObjectPtr,
}

impl WxMenu {

    pub fn new() -> Self {
        unsafe {
            WxMenu { menu : wx_create_menu() }
        }
    }

    pub fn raw(&self) -> WxObjectPtr {
        self.menu
    }

    pub fn append(&self, menu_id : u64) -> WxMenuItem {
        unsafe {
            let mi = wx_menu_append(self.menu, menu_id);
            WxMenuItem::new(mi)
        }
    }
}

// --------------------------------------------------------------------
// WxMenuItem

#[derive(Debug)]
pub struct WxMenuItem{
    menu_item : WxObjectPtr,
}

impl WxMenuItem {

    pub fn new( wx_menu_item : WxObjectPtr) -> Self {
        WxMenuItem { menu_item : wx_menu_item }
    }

    pub fn raw(&self) -> WxObjectPtr {
        self.menu_item
    }

    pub fn append(&self, menu_id : u64) {
        unsafe {
            wx_menu_append(self.menu_item, menu_id);
        }
    }

}

// --------------------------------------------------------------------
// WxMenuBar

#[derive(Debug)]
pub struct WxMenuBar{
    menu_bar : WxObjectPtr,
}

impl WxMenuBar {

    pub fn new() -> Self {
        unsafe {
            WxMenuBar { menu_bar : wx_create_menu_bar() }
        }
    }

    pub fn raw(&self) -> WxObjectPtr {
        self.menu_bar
    }

    pub fn append(&self, menu : &WxMenu, text : &str) {
        unsafe {
            let s = CString::new(text).expect("CString::new failed");
            wx_menu_bar_append(self.menu_bar, menu.raw(), s.as_ptr());
        }
    }
}

// --------------------------------------------------------------------
// WxCommandEvent

pub struct WxCommandEvent {}

// --------------------------------------------------------------------

unsafe extern "C" fn on_init(){
    let s = CString::new("Hello from rust").expect("CString::new failed");
    let f = wx_create_frame(s.as_ptr(), 50, 50, 450, 340);
    let wf = WxFrame::new(f);
    wf.init();
    INSTANCE.set(wf).unwrap();
}

pub fn wx_widgets_main() {

    unsafe {
        init_wx_widgets(0, 0, 0, 0, on_init);
        let t = 0;
    }
}
