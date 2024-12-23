fn main() {
    /*
        Type casting in Rust is done using the `as` keyword.
    */

    let a = 15;
    let b = (a as f64) / 2.0;
    println!("a: {}", a);
    println!("b: {}", b);

    // String or character types cannot be casted to integer or float types.
    // Characters cannot be type casted to strings and vice versa.
}
