# Rust

## installing Rust
- [installation page](https://www.rust-lang.org/tools/install)
- command for linux `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- note: Need to logout for adding path

- [source video](https://www.youtube.com/watch?v=rQ_J9WH6CGk)
## compile a file
- compile rust code `rustc <filename.rs>`
- run rust code './<filename>'

## Start a new project with cargo
- `cargo new helloProject`
- run the project `cargo run`
- projects compiles everytime when cargo runs
- other files need to compile everytime the code changes

## data types
- primitive data types
    - int, float, bool, char
### int
- signed and unsigend
- i8, i16 - i128: singned
- u8, - u128: unsigned

### float
- f32, f64

### boolien
- true/false
- `let is_snowing: bool = true;`

### char
- unicode character
- `let letter: char = 'a';`
- `println!('the fist letter of the alphabet: {}', a)`

## Compund data types
- array, tuples, slices, and strings (slice string)

### array
- homoginous data collection
- if integer only ingeger
- eg [1, 2, 3, 4]
- declaration `let array: [i32; 5] = [1, 2, 3, 4, 5];`
- `let fruits: [&str; 3] = ['apple', 'bananna', 'orange']`
- can slice like `fruit[0], fruit[1], fruit[2]
- index starts from 0

### tuple
- mixed data collection
- declaration `let human(String, i32, bool) = ('Alice'.to_string(), 30, false)`
- you can omit the datatypes that will work
- Alice is not a string it is a string slice
- `let my_mix_tuple = ("Kartos", 23, true, [1, 2, 3, 4, 5]);`

### Slices Vs String
- contigious sequence of elements
- adjusent places in memory
- `let number_slices:&[i32] = &[1, 2, 3, 4, 5]`
- `let book_slice:&[&String] = &[&"IT".to_string(), &"Harry Porter".to_string(), &"ZEN".to_string()]
- 'println!("Book Slices: {:?}", book_slice)
- string is mutable, can add values to string while slice can't
- all the varibles are immutable in rust
- need to add mut to make it mutable
- `let mut x:String = String::from("Hel, ")`

## Functions


