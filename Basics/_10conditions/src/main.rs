fn main() {
    let x = 10;
    if x > 10{
        println!("Value of x is greater than 10");
    } else if x == 10 {
        println!("Value of equal to 10");
    } else {
        println!("Value is less than 10");
    };
    statement();
}

// if condition in a let statement
fn statement(){
    let condition = true;
    let number = if condition {5} else {10};
    println!("number: {number}");
}
