

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

wx_on_init on_init_handler_ = NULL;
wx_object_ptr_t the_frame = NULL;

bool MyApp::OnInit()
{
    if (on_init_handler_)
        the_frame = on_init_handler_();

    //wxFrame * frame = new MyFrame( "Wx Widgets Dll 2", wxPoint(50, 50), wxSize(450, 340) );
    //frame->Show( true );
    return true;
}

MyFrame::MyFrame(const wxString& title, const wxPoint& pos, const wxSize& size)
    : wxFrame(NULL, wxID_ANY, title, pos, size)
{
    // on_create_handler_(reinterpret_cast<wx_object_ptr_t>(this));

    //wxMenu *menuFile = new wxMenu;
    //menuFile->Append(ID_Hello, "&Hello...\tCtrl-H",
    //    "Help string shown in status bar for this menu item");
    //menuFile->AppendSeparator();
    // menuFile->Append(wxID_EXIT);
    //wxMenu *menuHelp = new wxMenu;
    // menuHelp->Append(wxID_ABOUT);
    // wxMenuBar *menuBar = new wxMenuBar;
    // menuBar->Append( menuFile, "&File" );
    //menuBar->Append( menuHelp, "&Help" );
    // SetMenuBar( menuBar );
    //CreateStatusBar();
    //SetStatusText( "Welcome to wxWidgets!" );

    //Bind(wxEVT_COMMAND_MENU_SELECTED, &MyFrame::OnExit, this, wxID_EXIT);
    // Bind(wxEVT_COMMAND_MENU_SELECTED, &MyFrame::OnHello, this, ID_Hello);
    // Bind(wxEVT_COMMAND_MENU_SELECTED, &MyFrame::OnAbout, this, wxID_ABOUT);
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

extern "C" WX_WIDGETS_DLL2_API wx_object_ptr_t init_wx_widgets(
    unsigned __int64 hInstance,
    unsigned __int64 hPrevious,
    unsigned __int64 pCmdLine,
    __int32 nCmdShow,
    wx_on_init on_init_handler
)
{
    on_init_handler_ = on_init_handler;

    WinMain(
        reinterpret_cast<HINSTANCE>(hInstance),
        reinterpret_cast<HINSTANCE>(hPrevious),
        reinterpret_cast<wxCmdLineArgType>(pCmdLine),
        nCmdShow);

    return the_frame;
}


wx_object_ptr_t wx_create_frame(const char* text, unsigned __int32 point_x, unsigned __int32 point_y, unsigned __int32 size_w, unsigned __int32 size_h)
{
    return reinterpret_cast<wx_object_ptr_t>(
        new wxFrame(
            NULL, 
            wxID_ANY, 
            text, 
            wxPoint(point_x, point_y),
            wxSize(size_w, size_h)
            )
        );
}

wx_bool wx_frame_show(wx_object_ptr_t wx_frame, wx_bool show)
{
    return static_cast<wx_bool>(
        reinterpret_cast<wxFrame *>(wx_frame)->Show(static_cast<bool>(show))
        );
}

wx_object_ptr_t wx_create_menu()
{
    return reinterpret_cast<wx_object_ptr_t>(new wxMenu);
}

wx_object_ptr_t wx_menu_append(wx_object_ptr_t wx_menu, unsigned __int64 wx_menu_id)
{
    return reinterpret_cast<wx_object_ptr_t>(reinterpret_cast<wxMenu *>(wx_menu)->Append(wx_menu_id));
}

wx_object_ptr_t wx_create_menu_bar()
{
    return reinterpret_cast<wx_object_ptr_t>(new wxMenuBar);
}

wx_bool wx_menu_bar_append(wx_object_ptr_t wx_menu_bar, wx_object_ptr_t wx_menu, const char* text)
{
    return static_cast<wx_bool>(
            reinterpret_cast<wxMenuBar*>(wx_menu_bar)->Append(
                reinterpret_cast<wxMenu*>(wx_menu),
                text
            )
        );
}

wx_bool wx_menu_bar_append_1(wx_object_ptr_t wx_menu_bar, wx_object_ptr_t wx_menu)
{
    return static_cast<wx_bool>(
            reinterpret_cast<wxMenuBar*>(wx_menu_bar)->Append(
                reinterpret_cast<wxMenu*>(wx_menu),
                L"File"
            )
        );
}

wx_void wx_frame_set_menu_bar(wx_object_ptr_t wx_frame, wx_object_ptr_t wx_menu_bar)
{
    reinterpret_cast<wxFrame *>(wx_frame)->SetMenuBar(reinterpret_cast<wxMenuBar *>(wx_menu_bar));
    return 0;
}

wx_void wx_frame_bind_wxEVT_COMMAND_MENU_SELECTED(wx_object_ptr_t wx_frame, wx_object_ptr_t wx_menu, wx_frame_on_menu wx_on_menu, unsigned __int64 wx_menu_id, unsigned __int64 WxFrame)
{
    reinterpret_cast<MyFrame *>(wx_frame)->Bind(
        wxEVT_COMMAND_MENU_SELECTED,
        [WxFrame, wx_on_menu] (wxCommandEvent& event) { (wx_on_menu)(WxFrame, reinterpret_cast<wx_object_ptr_t>(&event)); },
        wx_menu_id, 
        wx_menu_id, 
        NULL);

    return 0;
}

wx_void wx_frame_close(wx_object_ptr_t wx_frame)
{
    reinterpret_cast<MyFrame *>(wx_frame)->Close();
    return 0;
}




