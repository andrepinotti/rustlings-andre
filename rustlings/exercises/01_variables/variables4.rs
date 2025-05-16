// DONE: Fix the compiler error.
fn main() {
    let x = 3;
    println!("Number {x}");

    // x = 5; // Don't change this line
    let x = 5;
    //FIXME -> ERROR: the variable x must be redeclare with let in the beginning
    println!("Number {x}");
}
