# RUST
    Rust lang book https://doc.rust-lang.org/book/

## 1.

#### Install rust on linux
    $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

    $ rustc --version
    
    $ rustup update

    $ rustup self uninstall     ////To uninstall Rust and rustup, run the following uninstall script from your shell:

    $ rustup doc => To get Local Documentation

#### To run rust programme 
    rustc main.rs
    
    if you get "error: linker `cc` not found" this error then "sudo apt install build-essential" run this command for linux OS

    If you want to stick to a standard style across Rust projects, you can use an automatic formatter tool called rustfmt to format your code in a particular style.

## Hello World
    fn main() {
        println!("Hello, world!");
    }

    println! calls a Rust macro.

    ! means that you’re calling a macro instead of a normal function, and that macros don’t always follow the same rules as functions.

## Cargo
Cargo is the Rust package manager.
- Cargo is Rust’s build system and package manager
- Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.
- To check the cargo version use `cargo --version` command.
- To create project using cargo use 
        `cargo new hello_cargo` command.
- To build project using cargo use `cargo build` command.
- To run the project using cargo use `cargo run` command.
- `cargo check` command is used to check if any error in project.
- `cargo help` to get more information.
- The last line, `[dependencies]`, is the start of a section for you to list any of your project’s dependencies.

    `$ cargo build`

    `$ ./target/debug/hello_cargo`
    
    Or
    
    `$ cargo run`

    `$ cargo check`

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`. Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

## 2.
### Guessing Game
```bash
use rand::Rng;

use std::cmp::Ordering;

use std::io;

fn main() {
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".red());
                break;
            }
        }
    }
}
```

- Cargo is that running the `cargo doc --open` command will build documentation provided by all of your dependencies locally and open it in your browser. 

- io input/output library
- Create a variable to store the user input, like this:  let mut guess = String::new();
- In Rust, variables are immutable by default. 
- To make a variable mutable, we add mut before the variable name.
- The equal sign (=) tells Rust we want to bind something to the variable now.
- The :: syntax in the ::new line indicates that new is an associated function of the String type
- An associated function is a function that’s implemented on a type, in this case String.
- This new function creates a new, empty string. 
- In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!
- stdin function from the io module, which will allow us to handle user input:
`io::stdin()
        .read_line(&mut guess)`
- The line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in
- A crate is a collection of Rust source code files.


## 3.
### Variables
- By default variables are immutable.
- When a variable is immutable, once a value is bound to a name, you can’t change that value.

### Constant
- constants are values that are bound to a name and are not allowed to change.
- You aren’t allowed to use mut with constants. 
- The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

### shadowing
- first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

### Data Type
- Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.

### Scalar Types:
- A scalar type represents a single value.
- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

#### Integers: 
- An integer is a number without a fractional component.
- So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127.
    
#### Compound Types:
- Compound types can group multiple values into one type.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.

## 4.
### Understanding Ownership
- Ownership is a set of rules that governs how a Rust program manages memory.
- The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. 
### Ownership Rules:
- Memory management.
- Ownership is nothing but the ways to mannage the memory.
- Ownership has control over the memory.
- Error Free 
- Faster runtime
- Smaller program size
- Stack are fixed in size.
  

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### Variable Scope
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

- The variable is valid from the point at which it’s declared until the end of the current scope.
- When s comes into scope, it is valid.
- It remains valid until it goes out of scope.
- References dose not take the ownership
- References are immutable.
- If we want to make it mutable then we need to pass the mutable reference.
- We can not borrow mutable references more than one time.


### Memory and Allocation
- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String.
- When we call `String::from`, its implementation requests the memory it needs.
- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
- Rust calls `drop` automatically at the closing curly bracket.
-  In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. 
- In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:

~
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
~

- When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.


let x = 5;
let y = x;

And

let s1 = String::from("hello");
let s2 = s1;

- Both are not the same.
- This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
- To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
- 

### Ownership and Functions

    fn main() {    
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

- 

 ### The Rules of References
 - At any given time, you can have either one mutable reference or any number of immutable references.
 - References must always be valid.

- The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
- We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.
`
 fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
`

 ### The Slice Type
 - Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 
 - A slice is a kind of reference, so it does not have ownership.


## 5.
### Struct, or structure: 

- A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

    ```bash
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    ```

    ```bash
    fn main() {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
    }
    ```



    ```bash 
    fn main() {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");

        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };
    }
    ```
    - Entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.

### Method Syntax
```bash
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50); //Specifying the width and height of the rectangle with a tuple

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect2);
    println!("rect1 is {:#?}", rect2);

    println!(
        "IMPliments The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 { //we want to borrow the struct rather than take ownership of it. 
    rectangle.width * rectangle.height
}

```
- impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.

- The println! macro call will now look like println!("rect1 is {rect1:?}");. Putting the specifier `:?` inside the curly brackets tells println! we want to use an output format called Debug. 
- In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.

## 6.
### Enums and Pattern Matching
- Enums allow you to define a type by enumerating its possible variants. 
### Defining an Enum
We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and V6. These are the variants of the enum:
```bash
enum IpAddrKind {
    V4,
    V6,
}
```
- IpAddrKind is now a custom data type that we can use elsewhere in our code.

### Enum Values
1. let four = IpAddrKind::V4;
2. let six = IpAddrKind::V6;


- Option<T> enum
```bash
enum Option<T> {
    None,
    Some(T),
}
```
- The <T> syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter

### The match Control Flow Construct
- Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

``` 
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
```

### Concise Control Flow with if let
- 
```rust
let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
```

- Using `if let` 

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } 

```


```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Using if let else 
```rust

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

```
- Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.  

##  7.
### Managing Growing Projects with Packages, Crates, and Modules
- A package can contain multiple binary crates and optionally one library crate.
- As a package grows, you can extract parts into separate crates that become external dependencies.

* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module

##### Packages and Crates
- A crate is the smallest amount of code that the Rust compiler considers at a time. 
- A crate can come in one of two forms: a binary crate or a library crate. 
- `Binary crates` are programs you can compile to an executable that you can run, such as a command-line program or a server.
- `Library crates` don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the `rand crate` 
- A `package` is a bundle of one or more crates that provides a set of functionality. 
- A package contains a Cargo.toml file that describes how to build those crates.
- Cargo passes the crate root files to rustc to build the library or binary.

#### Defining Modules to Control Scope and Privacy
- Create a new library named restaurant by running ```cargo new --lib restaurant```;
- mentioned that src/main.rs and src/lib.rs are called crate roots.

#### Grouping Related Code in Modules
- Code within a module is private by default.

#### Paths for Referring to an Item in the Module Tree
- An `absolute path` is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- A `relative path` starts from the current module and uses self, super, or an identifier in the current module.

## 8. Common Collections
- Collections can contain multiple values.
- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
- create new vector `let v: Vec<i32> = Vec::new();`
- In addition, you can conveniently use the + operator or the `format!` macro to concatenate String values.
- If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:

    ```bash
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    ```
At this point, s will be tic-tac-toe. With all of the + and " characters, it’s difficult to see what’s going on. For more complicated string combining, we can instead use the format! macro:

```bash
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
```
## 9.
#### Error Handling
- Rust groups errors into two major categories: recoverable and unrecoverable errors. 
- It has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error. 

## 10.
 #### Generic Types, Traits, and Lifetimes
  
 ```bash
    fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}
```
- Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. 

```bash
struct Point<T> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```
#### In Enum Definitions
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
#### In Method Definitions
```bash
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<U> Point<U> {
        fn x(&self) -> &U {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }
```

#### Traits: 
- A trait defines functionality a particular type has and can share with other types.

## 11.
#### Writing Automated Tests
- The  `#[test]` annotation: this attribute indicates this is a test function, so the test runner knows to treat this function as a test.
- The `assert!` macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.  
- If the value is true, nothing happens and the test passes. If the value is false, the assert! macro calls panic! to cause the test to fail. 
- 

