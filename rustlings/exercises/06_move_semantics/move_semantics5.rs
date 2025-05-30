#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase( data: &String) {
    let mut data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let mut data = "Rust is great!".to_string();

    println!("{}", data);

    get_char(data);


}
