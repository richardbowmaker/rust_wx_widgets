#pragma once

#ifdef WX_WIDGETS_DLL2_EXPORTS
#define WX_WIDGETS_DLL2_API __declspec(dllexport)
#else
#define WX_WIDGETS_DLL2_API __declspec(dllimport)
#endif




extern "C" WX_WIDGETS_DLL2_API void start_wxapp(
	unsigned __int64 hInstance,
	unsigned __int64 hPrevious,
	unsigned __int64 pCmdLine,
	__int32 nCmdShow
);

extern "C" WX_WIDGETS_DLL2_API void callback_test(unsigned __int64 pf);


typedef unsigned __int32 (* func)(unsigned __int32);

extern "C" WX_WIDGETS_DLL2_API void callback_test2(func f);

// ------------------------------------------------------------
// wxWidgets

typedef unsigned __int64 wx_object_ptr_t;
typedef unsigned __int64 wx_bool;
typedef unsigned __int64 wx_void;

const unsigned __int64 wxID_EXIT_ = 5006;
const unsigned __int64 wxID_ABOUT_ = 5014;

typedef wx_object_ptr_t (* wx_on_init)();

extern "C" WX_WIDGETS_DLL2_API wx_object_ptr_t init_wx_widgets(
	unsigned __int64 hInstance,
	unsigned __int64 hPrevious,
	unsigned __int64 pCmdLine,
	__int32 nCmdShow,
	wx_on_init on_init_handler
);

extern "C" WX_WIDGETS_DLL2_API wx_object_ptr_t wx_create_frame(const char* text, unsigned __int32 point_x, unsigned __int32 point_y, unsigned __int32 size_w, unsigned __int32 size_h);
extern "C" WX_WIDGETS_DLL2_API wx_bool wx_frame_show(wx_object_ptr_t wx_frame, wx_bool show);
extern "C" WX_WIDGETS_DLL2_API wx_object_ptr_t wx_create_menu();
extern "C" WX_WIDGETS_DLL2_API wx_object_ptr_t wx_menu_append(wx_object_ptr_t wx_menu, unsigned __int64 wx_menu_id);
extern "C" WX_WIDGETS_DLL2_API wx_object_ptr_t wx_create_menu_bar();
extern "C" WX_WIDGETS_DLL2_API wx_bool wx_menu_bar_append(wx_object_ptr_t wx_menu_bar, const wx_object_ptr_t wx_menu, const char* text);
extern "C" WX_WIDGETS_DLL2_API wx_bool wx_menu_bar_append_1(wx_object_ptr_t wx_menu_bar, const wx_object_ptr_t wx_menu);
extern "C" WX_WIDGETS_DLL2_API wx_void wx_frame_set_menu_bar(wx_object_ptr_t wx_frame, wx_object_ptr_t wx_menu_bar);
extern "C" WX_WIDGETS_DLL2_API wx_void wx_frame_close(wx_object_ptr_t wx_frame);

typedef wx_void (* wx_frame_on_menu)(wx_object_ptr_t wx_frame, wx_object_ptr_t wx_command_event);

extern "C" WX_WIDGETS_DLL2_API wx_void wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED(wx_object_ptr_t wx_frame, wx_object_ptr_t wx_menu, wx_frame_on_menu wx_on_menu, unsigned __int64 wx_menu_id);


// --------------------------------------------------------------

