

use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::errors::AppError;


#[derive(Debug)]
struct StructA{
    fa : i32,
}

impl StructA {

    fn new(a : i32) -> Self {
        Self{fa : a}
    }

    fn a(&self) -> i32 {
        self.fa
    }
}

#[derive(Debug)]
struct StructB {
    a: StructA,
    fb : i32, 
    // other fields...
}

impl StructB {

    fn new(a : i32, b : i32) -> Self {
        Self{a : StructA::new(a), fb : b}
    }

    fn b(&self) -> i32 {
        self.fb
    }
}

impl std::ops::Deref for StructB {
    type Target = StructA;
    fn deref(&self) -> &Self::Target {
        &self.a
    }
}


// -------------------------------------------------------------
// Function

struct Function {
    public : bool,
    stat : bool,
    mutable : bool,
    name : String,
    return_type : String,
    arguments : Vec<Argument>,
}

impl Function {
    fn new(public : bool, stat : bool, mutable : bool, name : &str, return_type : &str) -> Self {
        Self {
            public : public,
            stat : stat,
            mutable : mutable, 
            name : String::from(name), 
            return_type : String::from(return_type), 
            arguments : Vec::new() }
    }

    fn add_argument(&mut self, name : &str, typ : &str) {
        self.arguments.push(Argument::new(name, typ));
    }

    fn generate_code(&self) -> String {
        let mut s = String::new().to_owned();
        s.push_str("    ");

        if self.public { s.push_str("pub fn ") }
        else { s.push_str("fn "); }

        s.push_str(&self.name);
        s.push('(');

        if !self.stat {
            if self.mutable { s.push_str("&mut self, "); }
            else { s.push_str("&self, "); }
        }
        
        let mut d = "";
        for arg in &self.arguments {
            s.push_str(&format!("{}{}", d, arg));
            d = ", ";
        }

        s.push_str(")");

        if !self.return_type.is_empty() {
            s.push_str(&format!(" -> {}", self.return_type));
        }

        s
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.public { write!(f, "pub fn ").ok(); }
        else { write!(f, "fn ").ok(); }
        write!(f, "{}(", self.name).ok();
        if !self.stat {
            if self.mutable { write!(f, "&mut self, ").ok(); }
            else { write!(f, "&self, ").ok(); }
        } 
        for arg in &self.arguments {
            write!(f, "{}, ", arg).ok();
        }
        write!(f, ")").ok();
        if !self.return_type.is_empty() {
            write!(f, " -> {}", self.return_type).ok();
        }
        Ok(())
    }
}

// -------------------------------------------------------------
// Argument

struct Argument{
    name : String,
    typ : String
}

impl Argument {
    fn new(name : &str, typ : &str) -> Self {
        Self {
            name : String::from(name),
            typ : String::from(typ),
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.name, self.typ)
    }
}

// -------------------------------------------------------------
// Class

struct Class<'a> {
    parent      : Option<&'a Class<'a>>,
    name        : String,
    member_vars : Vec<Argument>,
    functions   : Vec<Function>,
}

impl<'a> Class<'a> {
    fn new(name : &str) -> Self {
        Self { parent : None, name : String::from(name), member_vars : Vec::new(), functions : Vec::new() }
    }

    fn set_parent(&mut self, parent : &'a Class<'a>) {
        self.parent = Some(parent);
    }

    fn add_member_var(&mut self, name : &str, typ : &str) {
        self.member_vars.push(Argument::new(name, typ));
    }

    fn add_member_function(&mut self, fun : Function) {
        self.functions.push(fun);
    }

    fn write_class(&self, f: &mut fmt::Formatter) {
        print!("{} : ", &self.name);
        if let Some(parent) = self.parent {
                parent.write_class(f);
        }
    }

    fn write_members(&self, f: &mut fmt::Formatter) {

        for var in &self.member_vars {
            writeln!(f, "  {},", var).ok();
        }

        if let Some(parent) = self.parent {
            parent.write_members(f);
        }
    }

    fn write_functions(&self, f: &mut fmt::Formatter) {

        for fnc in &self.functions {
            writeln!(f, "  {},", fnc).ok();
        }

        if let Some(parent) = self.parent {
            parent.write_functions(f);
        }
    }

    fn generate_code(&self, filename : &str, classes : Vec<&Class>) -> Result<(), AppError> {

        let mut file = File::create(filename)?;

        for class in &classes {
            let mut writer = BufWriter::new(&file);
            write!(&mut writer, "struct {} {{\n", class.name)?;
            
            for fnc in &class.functions {
                writeln!(&mut writer, "{}", fnc.generate_code())?;
            }

            write!(&mut writer, "}}\n\n")?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Class<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_class(f);
        writeln!(f, "").ok();
        self.write_members(f);
        self.write_functions(f);
        Ok(())
    }
}

// -------------------------------------------------------------
// create

pub fn create(filename : &str) {

    println!("Code generator");

    // let sb = StructB::new(1, 2);
    // println!("SB A: {}, b : {}", sb.a(), sb.b());

    let mut c1 = Class::new("Class1");
    
    c1.add_member_var("c1_a", "i32");
    c1.add_member_var("c1_b", "u64");

    let mut f1 = Function::new(true, false, false, "c1_fun1", "u64");
    f1.add_argument("arg1", "i16");
    f1.add_argument("arg2", "&str");
    c1.add_member_function(f1);

    let mut f2 = Function::new(true, false, true, "c1_fun2", "i32");
    f2.add_argument("arg1", "u64");
    c1.add_member_function(f2);

    let mut f3 = Function::new(false, true, false, "c1_fun3", "");
    f3.add_argument("arg1", "u64");
    f3.add_argument("arg2", "i64");
    f3.add_argument("arg3", "&str");
    f3.add_argument("arg4", "f32");
    c1.add_member_function(f3);



    let mut c2 = Class::new("Class2");

    c2.add_member_var("c2_a", "i32");
    c2.add_member_var("c2_b", "u64");

    f1 = Function::new(true, false, false, "c2_fun1", "u64");
    f1.add_argument("arg1", "i16");
    f1.add_argument("arg2", "&str");
    c2.add_member_function(f1);

    f2 = Function::new(true, false, true, "c2_fun2", "i32");
    f2.add_argument("arg1", "u64");
    c2.add_member_function(f2);

    f3 = Function::new(false, true, false, "c2_fun3", "");
    f3.add_argument("arg1", "u64");
    f3.add_argument("arg2", "i64");
    f3.add_argument("arg3", "&str");
    f3.add_argument("arg4", "f32");
    c2.add_member_function(f3);

    let cc2 = c2;
    
    c1.set_parent(&cc2);

    println!("{}", c1);

    let cc1 = c1;

    let mut classes : Vec<&Class> = Vec::new();
    classes.push(&cc1);
    classes.push(&cc2);
    

  
    match cc1.generate_code(r"D:\projects\rust_and_c\test_code.rs", classes) {
        Ok(_) => (),
        Err(_) => (),
    }

    
}




