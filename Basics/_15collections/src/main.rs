fn main() {
    vector();
}

fn vector(){
    let _v: Vec<i32> = Vec::new();
    // a dynamic array that can grow or shrink
    let _a_vector = vec![1, 2, 3, 4];
    // vec! a macro to initialize a vector
    let mut numbers = Vec::new();
    numbers.push(5);
    numbers.push(6);
    numbers.push(7);
    numbers.push(8);
    numbers.push(9);
    numbers.push(10);
    println!("Vector: {:?}", numbers);

}