fn main() {
    /*
        There are few ways to print text in Rust. The most common way is to use the println! macro.
        The println! macro prints text to the console and adds a newline at the end.
        The println! macro is similar to the print function in Python.
    */

    // The println! macro is used to print text to the console it appends a newline at the end
    println!("Hello, world!");

    // The print! macro is used to print text to the console without appending a newline at the end
    print!("Hello, ");
    print!("world!");

    // The eprintln! macro is used to print text to the standard error stream
    eprintln!("\nHello, error!");

    // The eprint! macro is used to print text to the standard error stream without appending a newline at the end
    eprint!("Error, ");

    // When using print! and eprint! they are printed on the same line even if you have a print! at
    // the start and eprint! at the end they will be printed on the same line at the end of the program
    println!("Hello from the other side!");
    eprint!("Error from the other side!");

    eprint!("Error from the other side! in same line");
    print!("Hello from the other side! in same line");
}
