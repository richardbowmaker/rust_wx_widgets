
// https://crates.io/

mod wx_widgets;
mod errors;
mod utilities;
mod main_frame;
mod code_generator;


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

fn main() {
    println!("wx widgets");
    code_generator::create(&"");

    // let d = Animal::<Dog>::new();
    // d.bark();
    
    // let sb = StructB::new(1, 2);
    // println!("{:?}", sb); 
    // println!("a {} b {}", sb.a(), sb.b()); 
  
    
    // main_frame::main();
    return ()
}


