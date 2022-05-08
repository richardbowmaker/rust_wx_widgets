

use std::default::Default;
use std::fmt;

use crate::errors::AppError;



// -----------------------------------------------
// wx code data struct
pub struct WxCodeData {
    rust_templates_dir      : String,
    rust_code_dir           : String,
    cpp_templates_dir       : String,
    cpp_code_dir            : String,
    template_start_marker   : String,
    template_end_marker     : String,
    classes                 : Vec<Class>,
}

impl fmt::Display for WxCodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "wx code database")?;
        for c in &self.classes {
            writeln!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl WxCodeData {
    pub fn new() -> Self {
        Self { 
            rust_templates_dir      : String::from(r"D:\projects\rust_and_c\wx_widgets_rust\generated\rust\templates"),
            rust_code_dir           : String::from(r"D:\projects\rust_and_c\wx_widgets_rust\generated\rust\code"),
            cpp_templates_dir       : String::from(r"D:\projects\rust_and_c\wx_widgets_rust\generated\cpp\templates"),
            cpp_code_dir            : String::from(r"D:\projects\rust_and_c\wx_widgets_rust\generated\cpp\code"),
            template_start_marker   : String::from("///@Template_start"),
            template_end_marker     : String::from("///@Template_end"),
            classes                 : Vec::new(),
        }
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


