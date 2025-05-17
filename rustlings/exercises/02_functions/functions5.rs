// DONE: Fix the function body without changing the signature.
// ERROR: append 'return' in line 4
fn square(num: i32) -> i32 {
    return num * num;
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}