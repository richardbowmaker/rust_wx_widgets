
use std::borrow::Borrow;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;


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

impl Drop for Function {
    fn drop(&mut self) {
        println!("dropped function {}", &self);
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
    member_fns : Vec<Function>,
}

impl Class {
    fn new(name : &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { parent : None,name : String::from(name), member_vars : Vec::new(), member_fns : Vec::new() }))
    }

    fn set_parent(&mut self, parent : Rc<RefCell<Class>>) {
        self.parent = Some(parent);
    }

    fn add_member_var(&mut self, name : &str, typ : &str) {
        self.member_vars.push(Argument::new(name, typ));
    }

    fn add_member_function(&mut self, fun : Function) {
        self.member_fns.push(fun);
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Class : {},", self.name).ok();

        let op = &self.parent;
        while let Some(p) = op {
            let x = p.borrow_mut();
            writeln!(f, " : {},", x.name).ok();
            let op = &x.parent;
        }

        for var in &self.member_vars {
            writeln!(f, "  {},", var).ok();
        }

        for fun in &self.member_fns {
            writeln!(f, "  {}", fun).ok();
        }
        Ok(())
    }
}

impl Drop for Class {
    fn drop(&mut self) {
        println!("dropped class {}", &self);
    }
}


// -------------------------------------------------------------
// create

pub fn create(filename : &str) {

    println!("Code generator");

    // let c = Class::new("MyClass");
    // let y = &c.borrow();

    let a = Rc::new(RefCell::new(1i32));
    // a.borrow();
  //  let c : &i32 = a.borrow_mut();




    // {
    //     let x = c.borrow_mut();
    //     x.add_member_var("fa", "i32");
    //     x.add_member_var("fb", "u64");

    //     let mut f1 = Function::new(true, false, false, "Fun1", "u64");
    //     f1.add_argument("arg1", "i16");
    //     f1.add_argument("arg2", "&str");
    //     x.add_member_function(f1);
 
    //     let mut f2 = Function::new(true, false, true, "Fun2", "i32");
    //     f2.add_argument("arg1", "u64");
    //     x.add_member_function(f2);

    //     let mut f3 = Function::new(false, true, false, "Fun3", "");
    //     f3.add_argument("arg1", "u64");
    //     f3.add_argument("arg2", "i64");
    //     f3.add_argument("arg3", "&str");
    //     f3.add_argument("arg4", "f32");
    //     x.add_member_function(f3);

    // }

 
    //println!("{}", c.
}




