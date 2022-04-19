
use std::ffi::CString;
use std::os::raw::c_char;
use once_cell::sync::OnceCell;

// string from C to rust
// https://stackoverflow.com/questions/50437953/provide-char-argument-to-c-function-from-rust


type WxObjectPtrT = u64;
type WxFunctionPtrT = u64;
type WxBoolT = u64;
type WxVoidT = u64;

pub const WX_ID_EXIT : u64 = 5006;
pub const WX_ID_ABOUT : u64 = 5014;

type OnInitEventT = fn();
type OnInitEventExternT = unsafe extern "C" fn(on_init : WxFunctionPtrT);
type OnMenuEventExternT = unsafe extern "C" fn(wx_frame : WxObjectPtrT, wx_command_event : WxObjectPtrT, wx_handler : WxFunctionPtrT) -> WxVoidT;

extern "C" {
//     fn init_wx_widgets_extern(hInstance : u64, hPrevious : u64, pCmdLine : u64, nCmdShow : i32,  on_init_handler : OnInitEventExternT) -> WxObjectPtrT;
    fn init_wx_widgets_extern(hInstance : u64, hPrevious : u64, pCmdLine : u64, nCmdShow : i32, on_init_extern : OnInitEventExternT, on_init : OnInitEventT) -> WxObjectPtrT;
    fn wx_create_frame_extern(text : *const c_char, point_x : u32, point_y : u32 , size_w : u32 , size_h : u32 ) -> WxObjectPtrT;
    fn wx_frame_show_extern(wx_frame : WxObjectPtrT, show : WxBoolT) -> WxBoolT;
    fn wx_create_menu_extern() -> WxObjectPtrT;
    fn wx_menu_append_extern(wx_menu : WxObjectPtrT, wx_menu_id : u64) -> WxObjectPtrT;
    fn wx_create_menu_bar_extern() -> WxObjectPtrT;
    fn wx_menu_bar_append_extern(wx_menu_bar : WxObjectPtrT, wx_menu : WxObjectPtrT, text : *const c_char) -> WxVoidT;
    fn wx_frame_set_menu_bar_extern(wx_frame : WxObjectPtrT, wx_menu_bar : WxObjectPtrT) -> WxVoidT;
    fn wx_frame_close_extern(wx_frame : WxObjectPtrT) -> WxVoidT;
    fn wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED_extern(wx_frame : WxObjectPtrT, wx_menu : WxObjectPtrT, wx_on_menu_handler : OnMenuEventExternT, wx_menu_id : u64, handler : WxFunctionPtrT) -> WxVoidT;
}

// ---------------------------------------------------------------------
// WxFrame

static INSTANCE: OnceCell<WxFrame> = OnceCell::new();

type OnMenuEventHandlerT = fn(frame : &WxFrame, event : &WxCommandEvent);

#[derive(Debug)]
pub struct WxFrame{
    main_frame : WxObjectPtrT,
}

impl WxFrame {
    pub fn instance() -> &'static WxFrame {
        INSTANCE.get().expect("WxFrame is not initialized")
    }
}

impl WxFrame {

    pub fn new(frame : WxObjectPtrT) -> Self {
        WxFrame { main_frame : frame }
    }

    pub fn create(text : &str, top : u32, left : u32, width : u32, height : u32) -> Self {
        unsafe {
            let t = CString::new(text).expect("WxFrame::create() CString::new failed");
            let f = wx_create_frame_extern(t.as_ptr(), 50, 50, 450, 340);
            WxFrame::new(f)
        }
    }

    pub fn show(&self, show : bool) {
        unsafe {
            wx_frame_show_extern(self.main_frame, show as WxBoolT);
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

    pub fn bind_menu_event_handler(&self, menu_item : &WxMenuItem, wx_menu_id: u64, handler : OnMenuEventHandlerT) {
        unsafe {
            wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED_extern(
                self.main_frame, 
                menu_item.raw(), 
                WxFrame::on_menu_event_handler_extern, 
                wx_menu_id,
        handler as WxFunctionPtrT);
        }
    }

    unsafe extern "C" fn on_menu_event_handler_extern(frame : WxObjectPtrT, event : WxObjectPtrT, wx_handler : WxFunctionPtrT) -> WxVoidT {
        let f = WxFrame::new(frame);
        let h : OnMenuEventHandlerT = std::mem::transmute(wx_handler);
        let e = WxCommandEvent::new(event);
        h(&f, &e);
        0
    }

}

// --------------------------------------------------------------------
// WxMenu

#[derive(Debug)]
pub struct WxMenu{
    menu : WxObjectPtrT,
}

impl WxMenu {

    pub fn new() -> Self {
        unsafe {
            WxMenu { menu : wx_create_menu_extern() }
        }
    }

    pub fn raw(&self) -> WxObjectPtrT {
        self.menu
    }

    pub fn append(&self, menu_id : u64) -> WxMenuItem {
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
    menu_item : WxObjectPtrT,
}

impl WxMenuItem {

    pub fn new( wx_menu_item : WxObjectPtrT) -> Self {
        WxMenuItem { menu_item : wx_menu_item }
    }

    pub fn raw(&self) -> WxObjectPtrT {
        self.menu_item
    }

    pub fn append(&self, menu_id : u64) {
        unsafe {
            wx_menu_append_extern(self.menu_item, menu_id);
        }
    }
}

// --------------------------------------------------------------------
// WxMenuBar

#[derive(Debug)]
pub struct WxMenuBar{
    menu_bar : WxObjectPtrT,
}

impl WxMenuBar {

    pub fn new() -> Self {
        unsafe {
            WxMenuBar { menu_bar : wx_create_menu_bar_extern() }
        }
    }

    pub fn raw(&self) -> WxObjectPtrT {
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
    event : WxObjectPtrT,
}

impl WxCommandEvent {
    pub fn new(e : WxObjectPtrT) -> Self {
        WxCommandEvent { event : e }
    }
}

// --------------------------------------------------------------------

unsafe extern "C" fn on_init_extern(on_init : WxFunctionPtrT) {
    let f : OnInitEventT = std::mem::transmute(on_init);
    f();
}

pub fn initialise(on_init : OnInitEventT)
{
    unsafe {
        init_wx_widgets_extern(
            0, 
            0, 
            0, 
            0, 
            on_init_extern as OnInitEventExternT, 
            on_init as OnInitEventT);
    }
}



