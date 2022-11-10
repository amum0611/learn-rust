/*

    Day 03: Functions, Data Types, Statements, Expressions

    $ cd ~/learn-rust
    $ cargo new day-03
    $ vi main.rs                    #copy the following code
    $ cargo run                    
*/

fn main() {
    let arr = [1, 2, 3, 4, 5];      // array of same type
    let tup = (123, 543.23, 4);     // tuple of different type

    let _total = calculate_tuple(tup, true, 'A', "Message One");
    let _total = calculate_tuple(tup, false, 'A', "Message One");
    println!("Total Tuple: {}", _total);    // we can print like this just like a standard string formatter
    
    let _total = calculate_array(arr);      
    println!("Total Array: {_total}")       // print like string interpolation
}

// fucntions with parameters
fn calculate_tuple(tup: (i32, f32, u8), flag: bool, _letter: char, msg: &str) -> f32 {

    println!("Entered values are: TUPLE [{:?}], Boolean[{flag}], String[{msg}]", tup);      // tuples cannot be formatted with the defult formatter; hence using a debug formatter

    let first_value_in_tuple = tup.0;
    let second_value_in_tuple = tup.1;
    let third_value_in_tuple = tup.2;

    if flag {
        println!("First Value in the TUPLE is {first_value_in_tuple}");   
    }

    first_value_in_tuple as f32 + second_value_in_tuple + third_value_in_tuple as f32   // The value is returned. No semi-colon

}

fn calculate_array(arr: [u8; 5]) -> u8 {

    let total = {   // This is an expression and not a statement
        let x: u8 = arr.iter().sum();
        x + 10  // The value is returned. No semi-colon
    };

    total   // The value is returned. No semi-colon
}


/*

NOTES:

    1. Rust doesn't do numneric promotions; so we need to perform conversion as shown in line 36.
    2. `main` function is the entry point.
    3. Rust code uses snake case as the conventional style for function and variable names.
    4. In function signatures, you must declare the type of each parameter.
    5. Rust contnins both statements and expressions. 
        * Statements are instructions that perform some action and do not return a value. 
        * Expressions evaluate to a resulting value
        * Expressions do not include ending semicolons
    6. Return values are not named. 
    7. We must declare functionis return type after an arrow (->). 
*/