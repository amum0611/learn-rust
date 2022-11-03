/*

    Day 01: Hello World

    Installation: https://www.rust-lang.org/learn/get-started - 
        - Rust is installed and managed by `rustup`, Rust's toolchain installer

    $ mkdir -p ~/learn-rust/day-01
    $ cd ~/learn-rust/day-01
    $ vi hello_world.rs             #copy the following code
    $ rustc hello_world.rs          #build the file using `rustc` the rust compiler 
    $ ./hello_world
*/

fn main() {
    println!("Hello, world!");
}

/*

NOTES: 

    1 - If your filename has more than one word, then you can use underscore to seperate them.
    2 - `rustup` also installs `cargo`, the Rust build tool and package manager. From day-02 onward, I will use `cargo` instead of `rustc`.
    3 - Rust is ahead-of-time compiled language. Once you compile it, you don't need Rust binaries to execute it. 
    4 - `println!` calls a Rust macro. If it had called a function instead, then I should have `println` without `!`. (more details on macro later)
*/
