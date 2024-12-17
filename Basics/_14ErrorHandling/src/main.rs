// example for Option
// fn main() {
//     let result = devide(10.0, 0.0);

//     match result{
//         Some(x) => println!("Result:{x}"),
//         None => println!("Can't divide by Zero!"),
//     }
// }

// fn devide(x:f32, y:f32)->Option<f32>{
//     if y == 0.0{
//         None
//     } else {
//         Some(x/y)
//     }
// }

// Example for Result
fn main(){
    let result = divide(10.0, 0.0);
    match result{
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

fn divide(x:f32, y:f32)-> Result<f32, String>{
    if y == 0.0{
        Err("Can't divide by 0".to_string())
    } else{
        // need to wrap inside ok function
        Ok(x/y)
    }
}