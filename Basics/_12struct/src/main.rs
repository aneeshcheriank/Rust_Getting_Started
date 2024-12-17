fn main() {
    let mut user = User{
        active: true,
        username: "Aneesh".to_string(),
        email: "aneesh@gmail.com".to_string(),
        sign_in_count: 10
    };
    user.email = "someotheremail@gmail.com".to_string();
    println!("{:#?}", user);
    //{:#?} will do the pritty printing
}

// struct Book{
//     title: String,
//     author: String,
//     pages: u16,
//     available: bool
// }

#[derive(Debug)] // Automatically implements the Debug trait for User
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
