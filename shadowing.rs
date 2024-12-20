fn main() {
    // Variable shadowing is a technique in which a variable declared within a certain scope has
    // the same name as a variable declared in an outer scope.

    let outer_variable = 10;
    {
        let outer_variable = 20;
        println!("Outer variable: {}", outer_variable);
    }
    println!("Outer variable: {}", outer_variable);
}
