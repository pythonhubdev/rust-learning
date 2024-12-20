fn main() {
    // Tuples in Rust are used to store multiple values of different types
    // A tuple  in Rust is Hetrogeneous, meaning it can store values of different types

    // Syntax 1:
    let person = ("John", "Doe", 30); // Not specifying the type of the tuple

    println!("Name: {} {}, Age: {}", person.0, person.1, person.2);

    // Syntax 2:
    let person_with_types: (&str, &str, i8) = ("John", "Doe", 30); // Specifying the type of the tuple

    println!(
        "Name: {} {}, Age: {}",
        person_with_types.0, person_with_types.1, person_with_types.2
    );

    // Unlike in Python tuples are mutable in Rust. We can use the mut keyword to make a tuple
    // mutable.

    let mut person_mutable = ("John", "Doe", 30);
    person_mutable.2 = 31;

    println!(
        "Name: {} {}, Age: {}",
        person_mutable.0, person_mutable.1, person_mutable.2
    );
}
