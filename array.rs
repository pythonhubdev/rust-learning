fn main() {
    // Compound types are used to group multiple values into one type
    // Rust has two primitive compound types: Tuple and Array

    // Tuple
    // A tuple is a collection of values of different types
    // A tuple is defined by enclosing the values in parentheses
    // A tuple can have a maximum of 12 elements
    let person: (&str, &str, i8) = ("John", "Doe", 30);
    println!("Name: {} {}, Age: {}", person.0, person.1, person.2);

    // Array
    // An array is a collection of values of the same type
    // An array is defined by enclosing the values in square brackets
    // An array has a fixed length
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);
    println!("First Number: {}", numbers[0]);
    println!("Second Number: {}", numbers[1]);
    println!("Third Number: {}", numbers[2]);
    println!("Fourth Number: {}", numbers[3]);
    println!("Fifth Number: {}", numbers[4]);

    // To make an array mutable, we need to use the mut keyword

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[0] = 10;

    println!("Numbers: {:?}", numbers);

    // Slicing an array
    // We can slice an array to get a subset of the elements
    let sliced_array: &[i32] = &numbers[1..4];

    println!("Sliced Array: {:?}", sliced_array);
}
