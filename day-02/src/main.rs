/*

    Day 02: Variables and Mutability

    $ cd ~/learn-rust
    $ cargo new day-02
    $ vi main.rs                    #copy the following code
    
    Run either, 
    $ cargo build                   #build the file using `cargo`, the Rust build tool and package manager 
    $ ./target/debug/day-02

    OR
    $ cargo run                     #An handy tool to iterate rapidly in development
*/

use std::io;

const DEFAULT_DISTANCE: u64 = 10;      //a constant with annotated variable type

fn main() {

    let name = "Captain Haddock";           //immutable variable (default) - type inference
    println!("Hello {name}!");              //string interporlatiion

    loop {  //similar to while loop
        let mut input = String::new();      //mutable variable

        println!("Enter a Valid Distance: ");

        io::stdin()
            .read_line(&mut input)          //string to store the user input, which is passed as reference
            .expect("An error occurred!");

        let input: u64 = match input.trim().parse() {   // see 4th note below
            Ok(num) => num,     // The Ok variant indicates the operation was successful, contains successfully generated value, which is returned. 
            Err(_) => continue, // loop until a valid number is entered
        };

        println!("You Entered: {input}. Default is {DEFAULT_DISTANCE}");    //string interporlatiion
        break;
    }
}

/*

NOTES:

    1. `use` keyword will bring libraries into scope that are not in the `prelude`.
    2. `Rust` variables are immutable by default. Keyword `mut` makes the vaariable mutable. 
    3. A `match` expression is made up of arms of patterns to match against and the code that should be run.
    4. `let input: u64 = match input...`
        a. Varaible `input` is already define in the scope.
        b. `Rust` allows to shadow previously defined variables with a new one.
        c. The colon `:` after `input` tells Rust we’ll annotate the variable’s type.
        b. There are name inbuilt number type. u64 means, unsigned 64-bit integer. 

*/