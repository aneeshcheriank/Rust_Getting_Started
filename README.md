# Rust
- [Rust Book](https://doc.rust-lang.org/book/)

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
- fn keyword
```
fn main(//arguments)-> <return_type>{
    //code
}
```

## Ownership
- garbage collection
- garbage collector stops program execution for feeing up the memory
- Evert vakyes gas a single owner
- borrow references
- Ownership rules
    - each value has an owner
    - only one owner at a time
    - when owner goes out of scope, the value will be dropped

## Borrowing and References
- borrowing value from the 
- ownership will not be transfered
- immutable / mutable references
    - immutable will not allow to modify the data
- & to borrow data
- we can have one mutable reference or many immutable referance

## Struct
- data type that groups toughter mutliple related vlaues (fields)
```
struct Person {
    name: String,
    age: u32,
    is_active: bool,
}
```

## Variables
- immutable by default
```
fn main() {
    println!("Hello, world!");
    let x: u16 = 5;
    println!("Value of x is {}", x)
    let x = 10
    // will generate an error can't assign an immutable variable twice
}
```
- define `let mut x:u8 = 5`, then you can change the value

## constants
- immutable
- can't use mut with constants
- `const Y:u8 = 10`
    - the name should be in upper case, otherwise generate a warning
    - the type should be there

## Shadowing
- 2 variables share same name
- 1st variable is shadowed by the second variable

## Comments
- readability
- // one line comment
- ctrl + / (vs code)
- /* */ multi-line comments

## if else condition
- control flows
    - conditions
    - repeating flows
```
fn main() {
    let x = 10;
    if x > 10{
        println!("Value of x is greater than 10");
    } else if x == 10 {
        println!("Value of equal to 10");
    } else {
        println!("Value is less than 10");
    }
}
```

## loops
- loop
    - unconditional loop
```
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```
- while
- for

## Structs
- struct
- impl
- function can return struct
- tuple struct
    ```
    struct Color(i32, i32, i32);
    let black: Color = Color(0, 0, 0);
    ```
- unit like struct
    - usally implemented when you need implement a trait but no need to store data
    ```
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;
    ```
## Enum
- bundle related data types
```
enum IpAddrKind{
    v4,
    v6,
}
```



