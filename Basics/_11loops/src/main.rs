//while loop
//for loop
//loop
fn main() {
    // loop function
    loop_loop();
    loop_label();
    // while funtions
    println!("While function");
    while_fn();

    // for loop
    println!("for loop");
    for_loop();
    iterate_elements();
}

fn while_fn(){
    let mut number = 3;
    while number <= 10{ // while loop
        println!("The number is {number}");
        number += 1;
    };
}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn iterate_elements(){
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn loop_loop(){
    let mut iteration = 10;
    loop{
        if iteration == 0{
            break;
        };
        println!("iteration number is {iteration}");
        iteration -= 1;
    };
}

fn loop_label(){
    let mut count = 0;
    'counting_up: loop { //labelled the loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remining = {remaining}");
            if remaining == 9{
                break;
            };
            if count == 2{
                break 'counting_up;
            };
            remaining -= 1;
        };
        count += 1;
    };
    println!("End count = {count}");
}
