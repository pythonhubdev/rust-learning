fn main() {
    /*
        -- Borrowing in Rust means to reference the original data binding or to share the data.
        -- Borrowing involves two variables referenced variable and referencing variable.
        -- Referenced variable - The variable that is being borrowed.
        -- Referencing variable - The variable that is borrowing the data.
    */

    /*
        -- Shared borrowing - Data is shared by single or multiple variables but cannot be altered. & is used to borrow the data.
        -- Mutable borrowing - Data is shared by single variable and can be altered. But it is not accessible by other variables at the same time. &mut is used to borrow the data.
    */

    // Dereferencing - Term used to refer to changing the value of the referenced variable using
    // it's address stored in the referring variable.

    // * - Dereferencing operator
    // * operand_1 = operand_2;

    let mut a = 15;
    let b = &mut a;

    // Dereferencing
    *b = 20;
    println!("Value of a after Dereferencing: {}", a);

    println!("{}", 3 + 4 - 9 / 6 * 6 ^ 8 & 3);
}
