
// https://crates.io/

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops:: {Deref, DerefMut};
use std::cell::Ref;


mod wx_widgets;
mod errors;
mod utilities;
mod main_frame;
mod code_generator;
mod code_generator1;



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

struct SA {
    val : i32,
    next : Option<Rc<RefCell<SA>>>,
}

impl SA {
    fn new(val : i32) -> Self {
        Self {val : val, next : None } 
    }

    fn set_next(&mut self, other : Rc<RefCell<SA>>) {
        self.next = Some(other);
    }

    // fn print(&self) {
    //     print!("{} : ", &self.val);
    //     if let Some(next) = &self.next {
    //         (*next).borrow_mut().print();
    //     }
    // }

    fn print(&self) {
        print!("{} : ", &self.val);
        if let Some(next) = &self.next {
            match (*next).try_borrow() {
                Ok(sa) => sa.print(),
                Err(_) => println!("error recursing struct"),
            }
        }
    }
}

struct SC {
    val : i32,
    next : Option<Rc<RefCell<SC>>>,
}

fn main() {
    println!("wx widgets");
    code_generator1::create("");
    // errors::main();

    // let sa1 = SA::new(1);
    // let sa2 = SA::new(2);
    // let rca1 = Rc::new(RefCell::new(sa1));
    // let rca2 = Rc::new(RefCell::new(sa2));

    // {
    //     let x = (*rca1).borrow();
    //     let y = (*rca1).borrow();

    //     println!("{}, {}", x.val, y.val);
    // }

    // {
    //     let mut x = (*rca1).borrow_mut();
    //     x.val = 3;
    //     x.set_next(rca2);
    // }

    // println!("{} ----", (*rca1).borrow().val);
    // (*rca1).borrow().print();


 






  
    // main_frame::main();

// ----------------------------------------------------

// let sa1 = SA::new(1);
// let sa2 = SA::new(2);
// let sa3 = SA::new(3);

// let rca1 = Rc::new(RefCell::new(sa1));

// {
//     let x = (*rca1).borrow();
//     let y = (*rca1).borrow();

//     println!("{}, {}", x.val, y.val);
// }

// {
//     let mut x = (*rca1).borrow_mut();
//     x.val = 3;
// }

// println!("{}", (*rca1).borrow().val);



// ----------------------------------------------------

    // let sb1 = SB { val : 1 };
    // let rca1 = RefCell::new(sb1);

    // {
    //     let x = rca1.borrow();
    //     let y = rca1.borrow();

    //     println!("{}, {}", x.val, y.val);
    // }

    // {
    //     let mut x = rca1.borrow_mut();
    //     x.val = 3;
    // }

    // println!("{}", rca1.borrow().val);


    


    return ()
}








