//  Define a global constant
const MAX_POINTS: u32 = 100_000;
fn main() {
    println!("Max Points: {}", MAX_POINTS);
    // Constants are always immutable
    // MAX_POINTS = 200_000; // This will throw an error
    // Constants can be declared in any scope
    const MAX_LEVEL: u32 = 100;
    println!("Max Level: {}", MAX_LEVEL);
    // Constants can be declared in any scope
    {
        const MAX_HEALTH: u32 = 100;
        println!("Max Health: {}", MAX_HEALTH);
    }
    // Constants can be declared in any scope
    // println!("Max Health: {}", MAX_HEALTH); // This will throw an error
}
