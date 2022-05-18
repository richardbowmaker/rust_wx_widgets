
use std::ffi::CString;
use std::os::raw::c_char;

type WxObjectPtrT = u64;
type WxFunctionPtrT = u64;
type WxBoolT = u64;
type WxVoidT = u64;

pub const WX_ID_EXIT : u64 = 5006;
pub const WX_ID_ABOUT : u64 = 5014;

type OnInitEventT = fn();
type OnInitEventExternT = unsafe extern "C" fn(on_init : WxFunctionPtrT);
type OnMenuEventExternT = unsafe extern "C" fn(wx_frame : WxObjectPtrT, wx_command_event : WxObjectPtrT, wx_handler : WxFunctionPtrT) -> WxVoidT;


#[derive(Debug)]
pub struct WxWindow{
    window : WxObjectPtrT,
}

#[derive(Debug)]
pub struct WxFrame{
    frame : WxObjectPtrT,
}



///#generated_start
impl WxFrame {
    pub fn CreateStatusBar(&self, number : Option<int>, style : Option<u64>, id : Option<i32>, name : Option<&str>) -> retun type {
        if let Some(number_) = number {
            if let Some(style_) = style {
                if let Some(id_) = id {
                    if let Some(name_) = name {
                        wx_frame_create_status_bar_extern_5(number, style, id, to_cstr!(name));
                    }
                    else {
                        wx_frame_create_status_bar_extern_4(number, style, id);
                    }
                }
                else {
                    wx_frame_create_status_bar_extern_3(number, style);
                }
            }
            else {
                wx_frame_create_status_bar_extern_2(number);
            }
        }
        else {
            wx_frame_create_status_bar_extern_1();
        }
    }
///#generated_end




