mod math_tools;

//use crate::math_tools::add_functions::add_five;
//use crate::math_tools::substract_functions::substract_five;
use crate::math_tools::{add_functions::add_five, substract_functions::substract_five};

fn main() {
    /*
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y = add_five(x);
    println!("y is {}", y);

    let z = substract_five(x);
    println!("z is {}", z);

    x = 70;
    println!("x is {}", x);
    */

    let mut s: String = String::from("hello world");
    let t: &mut String = &mut s;
    let f = &s;

    // It works
    println!("{}", t);
    println!("{}", s);

    // It doesn't work !?
    //println!("{}", s); //<- cannot borrow `s` as immutable because it is also borrowed as mutable
    //println!("{}", t);
}
