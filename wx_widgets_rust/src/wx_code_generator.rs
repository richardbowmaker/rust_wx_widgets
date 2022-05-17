

use std::default::Default;
use std::fmt;
use std::io::BufRead;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::io;
use std::io::Write;
use std::collections::HashMap;

use crate::errors::AppError;

// -----------------------------------------------
// wx code data struct
#[derive(Serialize, Deserialize, Debug)]
pub struct WxCodeData {
    rust_templates_dir      : String,
    rust_code_dir           : String,
    cpp_templates_dir       : String,
    cpp_code_dir            : String,
    template_start_marker   : String,
    template_end_marker     : String,


    arg_translation         : HashMap<String, String>,



    classes                 : Vec<Class>,
}

impl fmt::Display for WxCodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "wx code database")?;
        writeln!(f, "rust templates : {}", &self.rust_templates_dir)?;
        writeln!(f, "rust code : {}", &self.rust_code_dir)?;
        writeln!(f, "cpp templates : {}", &self.cpp_templates_dir)?;
        writeln!(f, "cpp code : {}", &self.cpp_code_dir)?;
        writeln!(f, "template start : {}", &self.template_start_marker)?;
        writeln!(f, "template end : {}", &self.template_end_marker)?;
        for c in &self.classes {
            writeln!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Default for WxCodeData {
    fn default() -> Self {

        let mut map  = HashMap::new();
        map.insert(String::from("wxWindow"),    String::from("WxWindow"));
        map.insert(String::from("wxWindowID"),  String::from("i32"));
        map.insert(String::from("wxString"),    String::from("&str"));
        map.insert(String::from("wxPoint"),     String::from("(i32, i32)"));
        map.insert(String::from("wxSize"),      String::from("(i32, i32)"));
        map.insert(String::from("long"),        String::from("i64"));

        Self {
            rust_templates_dir      : String::from(r"D:\projects\rust_and_c\wx_widgets_rust\generated\rust\templates"),
            rust_code_dir           : String::from(r"D:\projects\rust_and_c\wx_widgets_rust\generated\rust\code"),
            cpp_templates_dir       : String::from(r"D:\projects\rust_and_c\wx_widgets_rust\generated\cpp\templates"),
            cpp_code_dir            : String::from(r"D:\projects\rust_and_c\wx_widgets_rust\generated\cpp\code"),
            template_start_marker   : String::from("///#generated_start"),
            template_end_marker     : String::from("///#generated_end"),
            arg_translation         : map,
            classes                 : Vec::new(),
        }
    }
}

impl WxCodeData {
    pub fn new() -> Self {
        Self { 
            rust_templates_dir      : String::from(r""),
            rust_code_dir           : String::from(r""),
            cpp_templates_dir       : String::from(r""),
            cpp_code_dir            : String::from(r""),
            template_start_marker   : String::from(""),
            template_end_marker     : String::from(""),
            arg_translation         : HashMap::new(),
            classes                 : Vec::new(),
        }
    }

    pub fn init(filename : &str) -> Result<Self, AppError> {
        let s = fs::read_to_string(filename)?;
        let data : Self = serde_json::from_str(&s)?;
        Ok(data)
    }

    pub fn write_config_file(&self, filename : &str) -> Result<(), AppError> {
        let serialized = serde_json::to_string_pretty(&self)?;
        fs::write(filename, serialized)?;
        Ok(())
    }

    pub fn generate_source(data : &mut WxCodeData) -> Result<(), AppError> {
        Ok(())
    }

    pub fn add_class(&mut self, class : Class) {
        self.classes.push(class);
    }
}

// -----------------------------------------------
// Class struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    name    : String,
    methods : Vec<Method>,
}

impl Default for Class {
    fn default() -> Self {
        Self {
            name    : String::from(""), 
            methods : Vec::new(), } 
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "class {}", &self.name)?;
        for m in &self.methods {
            writeln!(f, "{}", m)?;
        }
        Ok(())
    }
}

impl Class {
    pub fn new(name : &str) -> Self {
        Self { name : name.to_owned(), methods : Vec::new() }
    }

    pub fn set_name(&mut self, name : &str) {
        self.name = name.trim().to_owned();
    }

    pub fn add_method(&mut self, method : Method) {
        self.methods.push(method);
    }
}

// -----------------------------------------------
// Method struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Method {
    is_virtual      : bool, 
    return_type     : String,
    is_ref          : bool,
    is_pointer      : bool,
    name            : String,
    is_constructor  : bool,
    is_destructor   : bool,
    arguments       : Vec<Argument>,
}

impl Default for Method {
    fn default() -> Self {
        Self {
            is_virtual      : false, 
            return_type     : String::from(""), 
            is_ref          : false, 
            is_pointer      : false, 
            name            : String::from(""), 
            is_constructor  : false,
            is_destructor   : false,
            arguments       : Vec::new(), } 
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_virtual { write!(f, "virtual ")?; }
        write!(f, "{}", &self.return_type)?;
        if self.is_ref { write!(f, "&")?; }
        if self.is_pointer { write!(f, "*")?; }
        write!(f, " {} (", &self.name)?;
        let mut sep = "";
        for arg in &self.arguments {
            write!(f, "{}{}", &sep, arg)?;
            sep = ", ";
        }
        write!(f, ")")?;
        if self.is_constructor { write!(f, " // constructor")?; }
        if self.is_destructor { write!(f, " // destructor")?; }
        Ok(())
    }
}

impl Method {
    pub fn set_is_virtual(&mut self, is_virtual : bool) {
        self.is_virtual = is_virtual;
    }

    pub fn set_return_type(&mut self, return_type : &str) {
        self.return_type = return_type.trim().to_owned();
    }

    pub fn set_is_ref(&mut self, is_ref : bool) {
        self.is_ref = is_ref;
    }

    pub fn set_is_pointer(&mut self, is_pointer : bool) {
        self.is_pointer = is_pointer;
    }

    pub fn set_name(&mut self, name : &str) {
        self.name = name.trim().to_owned();
    }

    pub fn set_is_constructor(&mut self, is_constructor : bool) {
        self.is_constructor = is_constructor;
    }

    pub fn set_is_destructor(&mut self, is_destructor : bool) {
        self.is_destructor = is_destructor;
    }

    pub fn add_argument(&mut self, arg : Argument) {
        self.arguments.push(arg);
    }
}

// -----------------------------------------------
// Argument struct
//
// e.g. const wxPoint &pos=wxDefaultPosition
#[derive(Serialize, Deserialize, Debug)]
pub struct Argument {
    is_const        : bool, 
    is_ref          : bool,
    is_pointer      : bool,
    type_           : String,
    name            : String,
    default_value   : String,
}

impl Default for Argument {
    fn default() -> Self {
        Self {
            is_const        : false, 
            is_ref          : false, 
            is_pointer      : false, 
            type_           : String::from(""), 
            name            : String::from(""), 
            default_value   : String::from(""),} 
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_const { write!(f, "const ")?; }
        write!(f, "{} ", &self.type_)?;
        if self.is_ref { write!(f, "&")?; }
        if self.is_pointer { write!(f, "*")?; }
        write!(f, "{}", &self.name)?;
        if self.default_value.len() > 0 {
            write!(f, "={}", &self.default_value)?;
        }
        Ok(())
    }
}

impl Argument {
    pub fn set_is_const(&mut self, is_const : bool) {
        self.is_const = is_const;
    }

    pub fn set_is_ref(&mut self, is_ref : bool) {
        self.is_ref = is_ref;
    }

    pub fn set_is_pointer(&mut self, is_pointer : bool) {
        self.is_pointer = is_pointer;
    }

    pub fn set_type(&mut self, type_ : &str) {
        self.type_ = type_.trim().to_owned();
    }

    pub fn set_name(&mut self, name : &str) {
        self.name = name.trim().to_owned();
    }

    pub fn set_default_value(&mut self, default_value : &str) {
        self.default_value = default_value.trim().to_owned();
    }

    pub fn has_default(&self) -> bool {
        self.default_value.len() > 0
    }
}

// ---------------------------------------------------
// code class
pub struct Code {
    code : Vec<String>,
    indent_size : usize,
    indent : String,
    line_no : usize,
}

impl Code {
    pub fn new() -> Self {
        Self { code : Vec::new(), indent_size : 0, indent : String::new(), line_no : 0usize }
    }

    pub fn add(&mut self, line : &str) {
        if self.line_no == self.code.len() {
            // start a new line
            self.code.push(String::new());
        }
        // add to last line
        if line.len() > 0 {
            if self.code[self.line_no].len() == 0 {
                self.code[self.line_no] = String::from(&self.indent) + &self.code[self.line_no] + line;
            } else {
                self.code[self.line_no] = String::from(&self.code[self.line_no]) + line;
            }
        }
    }

    pub fn add_line(&mut self, line : &str) {
        self.add(line);
        self.new_line();
    }

    pub fn new_line(&mut self) {
        if self.line_no == self.code.len() {
            self.code.push(String::new());
        }
        self.line_no += 1;
    }

    pub fn set_indent(&mut self, indent_size : usize) -> usize {
        self.indent_size = indent_size;
        self.indent = std::iter::repeat(" ").take(self.indent_size * 4).collect::<String>();
        self.indent_size
    }

    pub fn inc_indent(&mut self) -> usize {
        self.set_indent(self.indent_size + 1)
    }

    pub fn dec_indent(&mut self) -> usize {
        if self.indent_size > 0 { 
            self.set_indent(self.indent_size - 1);
        }
        self.indent_size
    }

    pub fn start_bracket(&mut self) {
        self.add_line(" {");
        self.inc_indent();
    }

    pub fn end_bracket(&mut self) {
        self.new_line();
        self.dec_indent();
        self.add_line("{");
    }

    pub fn test() {
        let mut code = Code::new();
        code.new_line();
        code.add("1");
        code.add("2");
        code.add("3");
        code.new_line();
        code.add("4");
        code.add("5");
        code.add("6");
        code.new_line();
        code.new_line();
        code.new_line();
        code.add_line("7");
        code.add_line("8");
        code.inc_indent();
        code.add("aaaaaaaaaaaaaaaaaaaaaa");
        code.start_bracket();
        code.add_line("bbbbbbbbbbbbbbbbbbbb");
        code.end_bracket();
        code.add_line("aaaaaaaaaaaaaaaaaaaaaa");
        code.dec_indent();
        code.add_line("c");
        code.add_line("c");
        code.add_line("c");
        println!("{}", code);
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "----------------------------------------------------------")?;
        writeln!(f, "Code, {} lines, line no {}, indent {}", self.code.len(), self.line_no, self.indent_size * 4)?;
        writeln!(f, "{}", "----------------------------------------------------------")?;
        for l in &self.code {
            writeln!(f, "{}", l)?;
        }
        writeln!(f, "{}", "----------------------------------------------------------")
    }
}

// ---------------------------------------------------
pub fn generate_rust_code(data : &WxCodeData, code : &mut Code) -> Result<(), AppError> {

    let class = &data.classes[0];
    let method = &class.methods[1];

    code.add(&format!("impl {}", &class.name));
    code.start_bracket();

    // function name
    code.add("pub fn ");
    if method.is_constructor {
        code.add("create(");
    }
    else {
        code.add(&method.name);
        code.add("(&self, ");
    }

    let mut sep = "";

    // function arguments
    for carg in &method.arguments {

        code.add(sep);
        sep = ", ";

        let rarg = data.arg_translation.get(&carg.type_)
            .ok_or(AppError::new(&format!("Unknown argument type {}", &carg.type_)))?;

        code.add(&carg.name);
        code.add(" : ");
        if !carg.default_value.is_empty() { code.add("Option<") }
        if carg.is_ref || carg.is_pointer { code.add("&"); }
        code.add(&rarg);
        if !carg.default_value.is_empty() { code.add(">") }
    }

    code.add(") -> retun type");
    code.start_bracket();


    // implementation
    generate_rust_function_implementation(data, code, &method.arguments, 0, &String::from("call_extern("))?;

    println!("{}", code);


    code.end_bracket();
        
    Ok(())
}

pub fn generate_rust_function_implementation(data : &WxCodeData, code : &mut Code, arguments : &Vec<Argument>, arg_index : usize, signature : &str) -> Result<(), AppError> {

/*    
    if let Some(name_) = name {
        frame = wx_frame_create_extern_5(parent_, id, to_cstr!(title), x, y, w, h, style_, to_cstr!(name_));
                                         ------------------------------------------------   
    }
    else {
        frame = wx_frame_create_extern_4(parent_, id, to_cstr!(title), x, y, w, h, style_);
    }
*/

    let mut line = String::from(signature);
    
    if arg_index >= arguments.len() {

        // add method signature so far
        code.add(&line);
        code.add_line(");");     
    }
    else {
        // separator
        if arg_index > 0 {
            line += ", ";
        }

        let current_arg = &arguments[arg_index];

        let mut n = if current_arg.has_default() {
            // if let Some(arg_) = arg {
            code.add_line(&format!("if let Some({}_) = {}", &current_arg.name, &current_arg.name));
            code.start_bracket();

            // argument name with underscore for option types
            String::from(&current_arg.name) + "_"
        }
        else {
            String::from(&current_arg.name)
        };

        // strings wrapped in to_cstr!() macro
        if current_arg.type_ == "wxString" {
            n = format!("to_cstr!({})", n);
        }

        println!("{}{}", line, n);
        generate_rust_function_implementation(data, code, arguments, arg_index + 1, &(String::from(&line) + &n))?;

        code.end_bracket();

        if current_arg.has_default() {
            code.dec_indent();
/*
            }
            else {
                frame = wx_frame_create_extern_4(parent_, id, to_cstr!(title), x, y, w, h, style_);
            }
*/
            code.add_line("}");
            code.add_line("else {");
            code.inc_indent();
            code.add(&line);
            code.add_line(");");
            code.end_bracket();
        }
    }
    Ok(())
}


/*
                                rust client type    rust call to c      imported type       exported type
                                                    conversion          rust                cpp extern "C"
--------------------------------------------------------------------------------------------------------------------                                                    
const wxString &title           &str                c_str!()            *const c_char       char*
wxWindow *parent                &WxWindow           none                *const c_void       void*
wxWindowID id                   i32                 none                c_int               int
const wxPoint 
    &pos=wxDefaultPosition      (i32, i32)          none, none          c_int, c_int        int


c call:

wxFrame(wxWindow *parent, wxWindowID id, const wxString &title, const wxPoint &pos=wxDefaultPosition, const wxSize &size=wxDefaultSize, long style=wxDEFAULT_FRAME_STYLE, const wxString &name=wxFrameNameStr)

rust client

pub fn create(parent : Option<&WxWindow>, id : i32, title : &str, pos : Option<(i32, i32)>, size : Option<(i32, i32)>, style : Option<i32>, name : Option<&str> ) -> Result<Self, AppError> {
    unsafe {
        let mut frame   : *const c_void = 0 as *const c_void;
        let mut parent_ : *const c_void = 0 as *const c_void;

        if let Some(p) = parent {
            parent_ = p.window;
        }

        if let Some((x, y)) = pos {
            if let Some((w, h)) = size {
                if let Some(style_) = style {
                    if let Some(name_) = name {
                        frame = wx_frame_create_extern_5(parent_, id, to_cstr!(title), x, y, w, h, style_, to_cstr!(name_));
                    }
                    else {
                        frame = wx_frame_create_extern_4(parent_, id, to_cstr!(title), x, y, w, h, style_);
                    }
                }
                else {
                    frame = wx_frame_create_extern_3(parent_, id, to_cstr!(title), x, y, w, h);
                }
            }
            else {
                frame = wx_frame_create_extern_2(parent_, id, to_cstr!(title), x, y);
            }
        }
        else {
            frame = wx_frame_create_extern_1(parent_, id, to_cstr!(title));
        }
        Ok(WxFrame::new(frame))
    }
}



import rust side

fn wx_frame_create_extern_1(parent : *const c_void, id : i32, title : *const c_char) -> *const c_void;
fn wx_frame_create_extern_2(parent : *const c_void, id : i32, title : *const c_char, point_x : i32, point_y : i32) -> *const c_void;
fn wx_frame_create_extern_3(parent : *const c_void, id : i32, title : *const c_char, point_x : i32, point_y : i32, size_w : i32 , size_h : i32) -> *const c_void;
fn wx_frame_create_extern_4(parent : *const c_void, id : i32, title : *const c_char, point_x : i32, point_y : i32, size_w : i32 , size_h : i32, style : i32) -> *const c_void;
fn wx_frame_create_extern_5(parent : *const c_void, id : i32, title : *const c_char, point_x : i32, point_y : i32, size_w : i32 , size_h : i32, style : i32, name : *const c_char) -> *const c_void;

export c++ side

extern "C" WX_WIDGETS_DLL2_API 

extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_1(void* parent, int id, char* title);
extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_2(void* parent, int id, char* title, int point_x, int point_y);
extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_3(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h);
extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_4(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h, int style);
extern "C" WX_WIDGETS_DLL2_API void* wx_frame_create_extern_5(void* parent, int id, char* title, int point_x, int point_y, int size_w, int size_h, int style, char* name);

c code

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









*/



// ---------------------------------------------------
pub fn update_rust_templates(data : &WxCodeData, code : &Code, filename : &str) -> Result<(), AppError> {
    let fsrc = fs::File::open(data.rust_templates_dir.to_owned() +  r"\" + filename)?;
    let src_lines = io::BufReader::new(fsrc).lines();

    let fdst = fs::File::create(data.rust_code_dir.to_owned() +  r"\" + filename)?;
    let mut bdst = io::BufWriter::new(fdst);

    for line in src_lines {
        let line = line?;
        if line == data.template_start_marker {
            bdst.write_all(&data.template_start_marker.as_bytes())?;
            bdst.write_all("\n".as_bytes())?;

            for c in &code.code {
                bdst.write_all(&c.as_bytes())?;
                bdst.write_all("\n".as_bytes())?;
            }
        }
        else {
            bdst.write_all(line.as_bytes())?;
            bdst.write_all("\n".as_bytes())?;
        }
    }
    Ok(())
}


