// Formatting uses placeholders to format text. Placeholders are defined using curly braces: {}.

fn main() {
    println!("{} days", 31); // {} is a placeholder for the 31 integer

    // We can use multiple placeholders in a single string
    println!("{} days, {} months", 31, 12); // 31 is the first placeholder, 12 is the second placeholder

    // We can also specify the order of the placeholders along with positional arguments
    println!("{0} days, {1} months", 31, 12); // 31 is the first placeholder, 12 is the second placeholder

    // We can repeat the same argument multiple times
    println!("{0} days, {0} months", 31); // 31 is the first placeholder, 31 is the second placeholder

    // We can use placeholder names
    println!("{days} days, {months} months", days=31, months=12); // days is the first placeholder, months is the second placeholder

    // We can use the placeholders for converting binary, hexadecimal, octal, and other number systems
    println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}", 10, 10, 10);

    // We can perform basic math operations in the placeholders
    println!("{} + {} = {}", 10, 15, 10 + 15);

    // Debug trait is used to print the value of a variable
    println!("Debug: {:?}", (10, 15, "Hello Rust"));
}