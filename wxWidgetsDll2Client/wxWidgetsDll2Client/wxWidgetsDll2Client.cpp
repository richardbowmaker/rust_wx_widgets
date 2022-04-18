// wxWidgetsDll2Client.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>

#include "wxWidgetsDll2.h"



// typedef wx_bool (* wx_on_create)(wx_object_ptr_t wx_frame)
// typedef wx_void (* wx_frame_on_menu)(wx_object_ptr_t wx_command_event);


wx_object_ptr_t frame = NULL;

wx_void on_menu(wx_object_ptr_t wx_frame, wx_object_ptr_t wx_command_event)
{
    wx_frame_close(frame);
    return 0;
}

wx_bool on_init()
{
    frame = wx_create_frame("Wx Widgets Client 2 CPP", 50, 50, 450, 340);
    wx_frame_show(frame, 1);
    wx_object_ptr_t menuFile = wx_create_menu();
    wx_object_ptr_t menuFileExt = wx_menu_append(menuFile, wxID_EXIT_);
    wx_menu_append(menuFile, wxID_ABOUT_);
    wx_object_ptr_t menuBar = wx_create_menu_bar();
    wx_menu_bar_append(menuBar, menuFile, "&File");
    wx_frame_set_menu_bar(frame, menuBar);
    wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED(frame, menuFileExt, on_menu, wxID_EXIT_);
    return 0;
}

int main()
{
    init_wx_widgets(0, 0, 0, 0, on_init);
}

