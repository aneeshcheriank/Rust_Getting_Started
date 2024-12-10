// Each value in Rust has a varible that is its owner
fn main(){
    let s1: String = String::from("Rust");
    let len = calculate_length(&s1); //passing reference &s1
    println!("Leght of '{}' is {}.", s1, len);
}

//why there is a -> in this function declaration
fn calculate_length(s: &str) -> usize {
    s.len()
}

// only one owner at a time
fn main(){
    let s1 = String::from("RUST");
    let s2 = s1; //trasfered the ownership

    //println("{}", s1); will generate an error
    println("{}", s2);
}

// when owner goes out of scope the value will be dropped
fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println("Lenght of '{}' is {}", s1, len)
}

// s1 goes out of scope and its value will be dropped
fn printLost(s: &str){
    println!("{}", &s1); //will generate an error
    // s1 not in scope
}

fn calculate_length(s: &str) -> usize {
    s.len()
}