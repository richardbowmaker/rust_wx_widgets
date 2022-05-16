// wxWidgetsDll2Client.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>

#include "wxWidgetsDll2.h"




void *frame = NULL;

void on_menu(void *wx_frame, void *wx_command_event, wx_function_ptr_t wx_handler)
{
    wx_frame_close_extern(frame);
}

bool on_init()
{
    frame = wx_create_frame_extern("Wx Widgets Client 2 CPP", 50, 50, 450, 340);
    wx_frame_show_extern(frame, 1);
    void *menuFile = wx_create_menu_extern();
   void *menuFileExt = wx_menu_append_extern(menuFile, wxID_EXIT_);
    wx_menu_append_extern(menuFile, wxID_ABOUT_);
    void *menuBar = wx_create_menu_bar_extern();
    wx_menu_bar_append_extern(menuBar, menuFile, "&File");
    wx_frame_set_menu_bar_extern(frame, menuBar);
 //   wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED_extern(frame, menuFileExt, on_menu, wxID_EXIT_, NULL);
    return 0;
}

int main()
{
    // init_wx_widgets_extern(0, 0, 0, 0, on_init);
}

