fn main() {
    let result = devide(10.0, 0.0);

    match result{
        Some(x) => println!("Result:{x}"),
        None => println!("Can't divide by Zero!"),
    }
}

fn devide(x:f32, y:f32)->Option<f32>{
    if y == 0.0{
        None
    } else {
        Some(x/y)
    }
}
