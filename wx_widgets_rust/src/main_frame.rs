
use crate::wx_widgets_if;


// https://stackoverflow.com/questions/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct-keeping-all-the-fiel


pub fn main()
{
        wx_widgets_if::initialise(on_init);
}

pub fn on_init() {

   let mf = wx_widgets_if::WxFrame::create(&"Rust WxWidgets", 100, 100, 300, 300);

   mf.show(true);
   let menu_file = wx_widgets_if::WxMenu::new();
   menu_file.append(wx_widgets_if::WX_ID_ABOUT);
   let menu_file_exit = menu_file.append(wx_widgets_if::WX_ID_EXIT);
   let menu_bar = wx_widgets_if::WxMenuBar::new();
   menu_bar.append(&menu_file, &"&File xxx");
   mf.set_menu_bar(&menu_bar);
   mf.bind_menu_event_handler(&menu_file_exit, wx_widgets_if::WX_ID_EXIT, on_menu_event_handler);
}

pub fn on_menu_event_handler(f : &wx_widgets_if::WxFrame, event : &wx_widgets_if::WxCommandEvent) {
    f.close();
}

pub struct MainFrame {

    wx_frame : wx_widgets_if::WxFrame,
}