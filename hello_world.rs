// A function is defined using the fn keyword
// The main function is a special function in Rust programs that is the entry point of the program
fn main() {
    println!("Hello, world!"); // println! is a macro that prints text to the console
}
/*
    The ! indicates that println! is a macro rather than a function.
    Macros are a more powerful feature than functions, but are more complex and have a different set of rules.
    1. They are used in metaprogramming
    2. They look like a function in C or C++ but work differently
    3. They get expanded to source code that get compiled with the rest of the program

    Macro -> Compiler expands -> Source code -> Compiled Code
*/

/*
    To run the program, use the rustc command to compile the program and then run the resulting executable.
    $ rustc hello_world.rs
    $ ./hello_world
    Hello, world!
*/
