fn main() {
    let x = 5;
    let x = x+1; //defined a varible with let keyworkd
    // mutability there is no 'let' keyword
    
    // define scope
    {
        let x = x*2;
        println!("The value of x in inner scope is {}", x);
    }
    // only the scond varible will be printed
    // not changing the value (immutable)
    // but shadowing the first variable?
    println!("Value of x in outer scope is {}", x);
}
