fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // DONE: Fix the function call.
    // ERROR -> We needed append the parameter in call_me function
    call_me(3);
}
