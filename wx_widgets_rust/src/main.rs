
// https://crates.io/

mod wx_widgets;
mod errors;


fn main() {
    println!("wx widgets");
    wx_widgets::wx_widgets_main();


    // let l1 = | n | do_f1(n);
    // println!("lambda 1 {}", l1(10));

    // let l2 = | n | do_f2(n, 10);

   
    // println!("lambda 2 {}", l2(2));

    return ()
}

fn do_f1(x : u64) -> u64 {
    x * x
}

fn do_f2(x : u64, y : u64) -> u64 {
    x * y
}


