fn main() {
    // Variables are like storage boxes that stores a piece of data of various data types.
    // A variable usually has a data type and a variable name associated to it.
    // In rust a variable is immutable i.e. they are not changeable by default, or you cannot reassign them

    // Use the keyword `let` to declare a variable
    let variable = "Hello World!";

    println!("Hey there, {}", variable);

    // Now what if we want to make a variable mutable?
    // We can use the `mut` keyword to make a variable mutable

    let mut mutable_variable = "Hello World!";
    println!("Hey there, {}", mutable_variable);

    mutable_variable = "Hello World! I am mutable now!";
    println!("Hey there, {}", mutable_variable);

    // Okay now let's assign multiple variables in a single line
    // We can do this by separating the variables with a comma and enclosing in brackets

    let (name, age) = ("John Doe", 25);
    println!("Hey there, my name is {} and I am {} years old", name, age);
}
