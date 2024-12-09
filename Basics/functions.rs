//Functions
fn main() {
    hello_world();
    tell_height(18);
    human_id("Aneesh", 40, 172.3);

    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // No `return` needed
    };

    println!("Result is: {}", _x);
    println!("Addition: {}", add(4, 6));
}

fn hello_world(){
    println!("Hello World!");
}

fn tell_height(height: i32){
    println!("My height is: {}", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm", name, age, height);
}

// Expressions and statements
// Experession: something that returns a value eg
    // 5
    // true & false
    // add(3, 5)
    // if condition
// Statement: anyting that does not return a value.

// anything outside function should be declared with const keyworkd
// not let
// const X = {
//     //code
// };
// all most all statements in Rust end with ;

fn add(a: i32, b:i32)-> i32{
    a+b
}