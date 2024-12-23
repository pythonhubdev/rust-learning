fn main() {
    // Arithmetic operators
    let mut a = 4;
    let mut b = 3;
    a = a + b;
    a = a * b;
    a = a - b;
    b = b - a;
    println!("a:{}", a);
    println!("b:{}", b);

    // Logical operators
    let mut a = false;
    let mut b = true;
    a = a && b || (!a);
    b = !b;
    println!("a:{}", a);
    println!("b:{}", b);

    // Comparison operators
    let a = 2;
    let b = 3;
    println!("Operand 1:{}, Operand 2:{}", a, b);
    println!("a > b:{}", a > b);
    println!("a < b:{}", a < b);
    println!("a >= b:{}", a >= b);
    println!("a <= b:{}", a <= b);
    println!("a == b:{}", a == b);
    println!("a != b:{}", a != b);

    //  BitWise operators
    // In Rust Right Shift operator i.e. >> is same as arithmetic right shift operator on signed
    // integers and logical right shift operator on unsigned integers.

    let mut a = 1;
    let mut b = 2;
    a = a & b;
    a = a << 1;
    b = b >> 3;
    println!("a: {}", a);
    println!("b: {}", b);
}
