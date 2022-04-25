
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;


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

struct Class {
    parent : Option<Rc<RefCell<Class>>>,
    name : String,
    member_vars : Vec<Argument>,
    functions : Vec<Function>,
}

impl Class {
    fn new(name : &str) -> Self {
        Self { parent : None, name : String::from(name), member_vars : Vec::new(), functions : Vec::new() }
    }

    fn new_wrapped(name : &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { parent : None,name : String::from(name), member_vars : Vec::new(), functions : Vec::new() }))
    }

    fn set_parent(&mut self, parent : Rc<RefCell<Class>>) {
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
        if let Some(parent) = &self.parent {
            match (*parent).try_borrow() {
                Ok(class) => class.write_class(f),
                Err(_) => println!("error recursing class"),
            }
        }
    }

    fn write_members(&self, f: &mut fmt::Formatter) {

        for var in &self.member_vars {
            writeln!(f, "  {},", var).ok();
        }

        if let Some(parent) = &self.parent {
            match (*parent).try_borrow() {
                Ok(class) => class.write_members(f),
                Err(_) => println!("error recursing class"),
            }
        }
    }

    fn write_functions(&self, f: &mut fmt::Formatter) {

        for fnc in &self.functions {
            writeln!(f, "  {},", fnc).ok();
        }

        if let Some(parent) = &self.parent {
            match (*parent).try_borrow() {
                Ok(class) => class.write_functions(f),
                Err(_) => println!("error recursing class"),
            }
        }
    }

}

impl fmt::Display for Class {
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

    let sb = StructB::new(1, 2);
    println!("SB A: {}, b : {}", sb.a(), sb.b());


 

    let c1 = Class::new_wrapped("Class 1");
    
    {
        let mut mc1 = c1.borrow_mut();

        mc1.add_member_var("c1_a", "i32");
        mc1.add_member_var("c1_b", "u64");
    
        let mut f1 = Function::new(true, false, false, "c1_fun1", "u64");
        f1.add_argument("arg1", "i16");
        f1.add_argument("arg2", "&str");
        mc1.add_member_function(f1);
    
        let mut f2 = Function::new(true, false, true, "c1_fun2", "i32");
        f2.add_argument("arg1", "u64");
        mc1.add_member_function(f2);
    
        let mut f3 = Function::new(false, true, false, "c1_fun3", "");
        f3.add_argument("arg1", "u64");
        f3.add_argument("arg2", "i64");
        f3.add_argument("arg3", "&str");
        f3.add_argument("arg4", "f32");
        mc1.add_member_function(f3);
    
    }

    let c2 = Class::new_wrapped("Class 2");
    {
        let mut mc2 = c2.borrow_mut();

        mc2.add_member_var("c2_a", "i32");
        mc2.add_member_var("c2_b", "u64");
    
        let mut f1 = Function::new(true, false, false, "c2_fun1", "u64");
        f1.add_argument("arg1", "i16");
        f1.add_argument("arg2", "&str");
        mc2.add_member_function(f1);
    
        let mut f2 = Function::new(true, false, true, "c2_fun2", "i32");
        f2.add_argument("arg1", "u64");
        mc2.add_member_function(f2);
    
        let mut f3 = Function::new(false, true, false, "c2_fun3", "");
        f3.add_argument("arg1", "u64");
        f3.add_argument("arg2", "i64");
        f3.add_argument("arg3", "&str");
        f3.add_argument("arg4", "f32");
        mc2.add_member_function(f3);
    
    }
    
    c1.borrow_mut().set_parent(c2);
    println!("{}", c1.borrow_mut());

    
}




