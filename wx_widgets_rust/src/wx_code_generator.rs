

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
}

// ---------------------------------------------------
// code class
pub struct Code {
    code : Vec<String>,
    indent_size : usize,
    indent : String,
}

impl Code {
    pub fn new() -> Self {
        Self { code : Vec::new(), indent_size : 0, indent : String::new() }
    }

    pub fn add(&mut self, line : &str) {
        if let Some(s) = self.code.pop() {
            self.code.push(s + line);
        }
        else {
            self.add_line(line);
        }
    }

    pub fn add_line(&mut self, line : &str) {
        self.code.push(self.indent.to_owned() + line);
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
}

// ---------------------------------------------------
pub fn generate_rust_code(data : &WxCodeData, code : &mut Code) -> Result<(), AppError> {

/*

impl WxFrame {


   pub fn create(text : &str, top : u32, left : u32, width : u32, height : u32) -> Self {
        unsafe {
            let t = CString::new(text).expect("WxFrame::create() CString::new failed");
            let f = wx_create_frame_extern(t.as_ptr(), 50, 50, 450, 340);
            WxFrame::new(f)
        }
    }


*/


    code.add_line(r#"impl WxFrame {"#);
    code.inc_indent();
    // code.add_line(r#"pub fn create(text : &str, top : u32, left : u32, width : u32, height : u32) -> Self {"#);
    // code.inc_indent();
    // code.add_line(r#"unsafe {"#);
    // code.add_line(r#"}"#);
    // code.dec_indent();
    // code.add_line(r#"}"#);
    // code.dec_indent();
 
    let class = &data.classes[0];
    let method = &class.methods[1];

    // function name
    code.add_line("");
    code.add("pub fn ");
    if method.is_constructor {
        code.add("create");
    }
    else {
        code.add(&method.name);
    }
    code.add("(&self, ");

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

    code.add(")");
    code.add_line("");
    code.dec_indent();
    code.add_line("}");
        
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
    &pos=wxDefaultPosition      (u32, u32)          none, none          c_int, c_int        int






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


