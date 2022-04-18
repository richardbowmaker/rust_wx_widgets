
// https://crates.io/

mod wx_widgets;
mod errors;
mod utilities;

#[derive(Debug)]
struct SS {
    val : i32,
}

impl SS {

    fn to_addr(&self) -> usize {
        // std::ptr::addr_of!(self) as usize
        let p1 : *const &SS = std::ptr::addr_of!(self);
        p1 as usize
    }

    fn from_addr<'b>(address: usize) -> &'b Self {
        unsafe { &*(address as *const &Self) }
    }
}



fn main() {
    println!("wx widgets");
    wx_widgets::wx_widgets_main();
    let t = 0;


    // let s1 = SS { val : 23 };

    // let p1 : *const SS = std::ptr::addr_of!(s1);

    // let n1 : usize = p1 as usize;
    // let n2 = s1.to_addr();
    // let n3 = utilities::to_addr(&s1);

    // let s2 = SS::from_addr(n2);
    // println!("We got back the value: {:?}!", s2);
    // let n6 = s2.to_addr();


    // let n4 = utilities::to_addr(&s2);
    // let p2 : *const SS = std::ptr::addr_of!(*s2);
    // let n5 : usize = p2 as usize;



    // let s3 : &SS = unsafe {
    //     utilities::from_addr(n2)
    // };
    // println!("We got back the value: {:?}!", s3);


 
  
    return ()
}


