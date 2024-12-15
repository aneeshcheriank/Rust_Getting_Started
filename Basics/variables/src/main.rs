fn main() {
    let mut x: u16 = 5;
    println!("Value of x is {}", x);
    x = 10;
    // will generate an error can't assign an immutable variable twice
    println!("Value of x is {}", x)
}


