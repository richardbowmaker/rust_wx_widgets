

// wxWidgets "Hello world" Program
// For compilers that support precompilation, includes "wx/wx.h".
#include "pch.h" 

#include <wx/wxprec.h>
#ifndef WX_PRECOMP
#include <wx/wx.h>
#endif

#include "wxWidgetsDll2.h"
#include <stdio.h>



extern "C" int WINAPI WinMain(HINSTANCE hInstance,   
    HINSTANCE hPrevInstance,                 
    wxCmdLineArgType WXUNUSED(lpCmdLine),     
    int nCmdShow);

void start_wxapp(
    unsigned __int64 hInstance,
    unsigned __int64 hPrevious,
    unsigned __int64 pCmdLine,
    __int32 nCmdShow)
{
    WinMain(
        reinterpret_cast<HINSTANCE>(hInstance),
        reinterpret_cast<HINSTANCE>(hPrevious),
        reinterpret_cast<wxCmdLineArgType>(pCmdLine),
        nCmdShow);
};



void callback_test(unsigned __int64 upf)
{

    wxMessageBox( "In C callback_test",
        "wxWidgets2 DLL", wxOK | wxICON_INFORMATION );

    void (*pf)();
    pf = reinterpret_cast<void (*)()>(upf);
    pf();

}

void callback_test2(func f)
{
    unsigned __int32 x = f(10);
    wchar_t buff[200];
    swprintf_s(buff, _countof(buff), L"result from callback test 2 %d", x);
    wxMessageBox(buff, L"wxWidgets2 DLL", wxOK | wxICON_INFORMATION);
}

class MyApp: public wxApp
{
public:
    virtual bool OnInit();
};

class MyFrame: public wxFrame
{
public:
    MyFrame(const wxString& title, const wxPoint& pos, const wxSize& size);

private: 

    void OnHello(wxCommandEvent& event);
    void OnExit(wxCommandEvent& event);
    void OnAbout(wxCommandEvent& event);

};

enum
{
    ID_Hello = 1
};



// wxIMPLEMENT_APP(MyApp);
wxIMPLEMENT_WX_THEME_SUPPORT 

extern "C" int WINAPI WinMain(HINSTANCE hInstance,   
    HINSTANCE hPrevInstance,                 
    wxCmdLineArgType WXUNUSED(lpCmdLine),     
    int nCmdShow)                            
{                                                                     
    wxDISABLE_DEBUG_SUPPORT();                                       

    /* NB: We pass NULL in place of lpCmdLine to behave the same as  */ 
    /*     Borland-specific wWinMain() above. If it becomes needed   */ 
    /*     to pass lpCmdLine to wxEntry() here, you'll have to fix   */ 
    /*     wWinMain() above too.                                     */ 
    return wxEntry(hInstance, hPrevInstance, NULL, nCmdShow);          
}   

wxIMPLEMENT_WXWIN_MAIN_BORLAND_NONSTANDARD
wxIMPLEMENT_APP_NO_MAIN(MyApp);

wx_on_init_extern on_init_extern_ = NULL;
wx_on_init on_init_ = NULL;

bool MyApp::OnInit()
{
    if (on_init_extern_)
        on_init_extern_(on_init_);
    return true;
}

MyFrame::MyFrame(const wxString& title, const wxPoint& pos, const wxSize& size)
    : wxFrame(NULL, wxID_ANY, title, pos, size)
{
}

void MyFrame::OnExit(wxCommandEvent& event)
{
    Close( true );
}

void MyFrame::OnAbout(wxCommandEvent& event)
{
    wxMessageBox( "This is a wxWidgets' Hello world sample",
        "About Hello World", wxOK | wxICON_INFORMATION );
}

void MyFrame::OnHello(wxCommandEvent& event)
{
    wxLogMessage("Hello world from wxWidgets!");
}

// ------------------------------------------------------------

extern "C" WX_WIDGETS_DLL2_API void init_wx_widgets_extern(
    unsigned __int64 hInstance,
    unsigned __int64 hPrevious,
    char *pCmdLine,
    int nCmdShow,
    void *on_init_extern,
    void *on_init
)
{
    int n = sizeof(int);

    on_init_extern_ = reinterpret_cast<wx_on_init_extern>(on_init_extern);
    on_init_ = reinterpret_cast<wx_on_init>(on_init);

    WinMain(
        reinterpret_cast<HINSTANCE>(hInstance),
        reinterpret_cast<HINSTANCE>(hPrevious),
        reinterpret_cast<wxCmdLineArgType>(pCmdLine),
        nCmdShow);
}

void *wx_create_frame_extern(const char* text, int point_x, int point_y, int size_w, int size_h)
{
    return reinterpret_cast<void *>(
        new wxFrame(
            NULL, 
            wxID_ANY, 
            text, 
            wxPoint(point_x, point_y),
            wxSize(size_w, size_h)
            )
        );
}

int wx_frame_show_extern(void *wx_frame, int show)
{
    return static_cast<int>(
        reinterpret_cast<wxFrame *>(wx_frame)->Show(static_cast<bool>(show))
        );
}

void *wx_create_menu_extern()
{
    return reinterpret_cast<void *>(new wxMenu);
}

void *wx_menu_append_extern(void *wx_menu, int wx_menu_id)
{
    return reinterpret_cast<void *>(reinterpret_cast<wxMenu *>(wx_menu)->Append(wx_menu_id));
}

void *wx_create_menu_bar_extern()
{
    return reinterpret_cast<void *>(new wxMenuBar);
}

int wx_menu_bar_append_extern(void *wx_menu_bar, void *wx_menu, const char* text)
{
    return static_cast<int>(
            reinterpret_cast<wxMenuBar*>(wx_menu_bar)->Append(
                reinterpret_cast<wxMenu*>(wx_menu),
                text
            )
        );
}

void wx_frame_set_menu_bar_extern(void *wx_frame, void *wx_menu_bar)
{
    reinterpret_cast<wxFrame *>(wx_frame)->SetMenuBar(reinterpret_cast<wxMenuBar *>(wx_menu_bar));
}

void wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED_extern(void *wx_frame, void *wx_menu, void *wx_on_menu, int wx_menu_id, wx_function_ptr_t wx_handler){

    wx_frame_on_menu wx_on_menu_ = reinterpret_cast<wx_frame_on_menu>(wx_on_menu);

    reinterpret_cast<wxFrame *>(wx_frame)->Bind(
        wxEVT_COMMAND_MENU_SELECTED,
        [wx_frame, wx_on_menu_, wx_handler] (wxCommandEvent& event) { (wx_on_menu_)(wx_frame, reinterpret_cast<void *>(&event), wx_handler); },
        wx_menu_id, 
        wx_menu_id, 
        NULL);
}

void wx_frame_close_extern(void *wx_frame)
{
    reinterpret_cast<wxFrame *>(wx_frame)->Close();
}

// new ---------------------

void* wx_frame_create_extern_1(void* parent, int id, char* title)
{
    return new wxFrame(reinterpret_cast<wxWindow *>(parent), id, title);
}

void* wx_frame_create_extern_2(void* parent, int id, char* title, int point_x, int point_y)
{
    return new wxFrame(reinterpret_cast<wxWindow *>(parent), id, title, wxPoint(point_x, point_y));
}

void* wx_frame_create_extern_3(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h)
{
    return new wxFrame(reinterpret_cast<wxWindow *>(parent), id, title, wxPoint(point_x, point_y), wxSize(size_w, size_h));
}

void* wx_frame_create_extern_4(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h, int style)
{
    return new wxFrame(reinterpret_cast<wxWindow *>(parent), id, title, wxPoint(point_x, point_y), wxSize(size_w, size_h), style);
}

void* wx_frame_create_extern_5(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h, int style, char* name)
{
    return new wxFrame(reinterpret_cast<wxWindow *>(parent), id, title, wxPoint(point_x, point_y), wxSize(size_w, size_h), style, name);
}





