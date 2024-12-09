fn main(){
    let book_slices: &[&String] = &[&"IT".to_string(), 
    &"Harry Porter".to_string(), &"ZEN".to_string()];

    println!("Book Slice: {:?}", book_slices);

    //String Vs String Slices(&Str)
    //String[growable, mutalbe, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says: {}", stone_cold);

    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice)
}

// fn print(){
//     println!("Slice: {}", slice);
// }
// will get and out of scope error