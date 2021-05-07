fn main() {
    // mut - mutable, allows for variables to be reassigned
    // let mut x = 45;

    // let x = 45; //  i32 assumes

    let mut x: u64 = 45; // u64 for unsigned, if you're not going to use neg number.
    let f: f32 = 6.7; // f32
    let b: bool = false;

    println!("the value of x is {}", x);

    x = 60;
    println!("the value of x is {}", x);
}
