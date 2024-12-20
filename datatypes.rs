fn main() {
    // Rust can automatically infer the type of a variable
    // But we can also explicitly define the type of a variable

    let number: i32 = 10; // i32 is a 32-bit signed integer
    println!("Number: {}", number);
    // There are primitive datatypes in Rust
    // Scalar -> Numeric -> Integer and Float
    // Scalar -> Non-numeric -> Boolean and Character
    // Compound -> Tuple and Array

    /*
        Integer
        i8: The 8-bit signed integer type.

        i16: The 16-bit signed integer type.

        i32: The 32-bit signed integer type.

        i64: The 64-bit signed integer type.

        u8: The 8-bit unsigned integer type.

        u16: The 16-bit unsigned integer type.

        u32: The 32-bit unsigned integer type.

        u64: The 64-bit unsigned integer type.

    */

    /*
        Float
        f32: The 32-bit floating point type.

        f64: The 64-bit floating point type.
    */

    /*
        String and Character
        String: A collection of characters. In Rust a string is enclosed in double quotes.
        Character: A single character. In Rust a character is enclosed in single quotes.

        Unlike other languages, in Rust a character occupies 4 bytes of memory. This is to support emoji and other languages.

        In Rust a String is defined by the keyword &str.
    */
}
