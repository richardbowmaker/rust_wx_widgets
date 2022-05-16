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

typedef void *wx_function_ptr_t;

const unsigned __int64 wxID_EXIT_ = 5006;
const unsigned __int64 wxID_ABOUT_ = 5014;

typedef void (* wx_on_init)();
typedef void (* wx_on_init_extern)(wx_on_init);

extern "C" WX_WIDGETS_DLL2_API void init_wx_widgets_extern(
	unsigned __int64 hInstance,
	unsigned __int64 hPrevious,
	char *pCmdLine,
	int nCmdShow,
	void *on_init_extern,
	void *on_init
);

extern "C" WX_WIDGETS_DLL2_API void *wx_create_frame_extern(const char* text, int point_x, int point_y, int size_w, int size_h);
extern "C" WX_WIDGETS_DLL2_API int wx_frame_show_extern(void *wx_frame, int show);
extern "C" WX_WIDGETS_DLL2_API void *wx_create_menu_extern();
extern "C" WX_WIDGETS_DLL2_API void *wx_menu_append_extern(void *wx_menu, int wx_menu_id);
extern "C" WX_WIDGETS_DLL2_API void *wx_create_menu_bar_extern();
extern "C" WX_WIDGETS_DLL2_API int wx_menu_bar_append_extern(void *wx_menu_bar, void *wx_menu, const char* text);
extern "C" WX_WIDGETS_DLL2_API void wx_frame_set_menu_bar_extern(void *wx_frame, void *wx_menu_bar);
extern "C" WX_WIDGETS_DLL2_API void wx_frame_close_extern(void *wx_frame);

typedef void (* wx_frame_on_menu)(void *wx_frame, void *wx_command_event, wx_function_ptr_t wx_handler);

extern "C" WX_WIDGETS_DLL2_API void wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED_extern
	(void *wx_frame, void *wx_menu, void *wx_on_menu, int wx_menu_id, wx_function_ptr_t wx_handler);

// new


extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_1(void* parent, int id, char* title);
extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_2(void* parent, int id, char* title, int point_x, int point_y);
extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_3(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h);
extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_4(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h, int style);
extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_5(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h, int style, char* name);




// --------------------------------------------------------------

