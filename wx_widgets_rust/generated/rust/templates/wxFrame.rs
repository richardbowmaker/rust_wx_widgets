
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
///#generated_end




