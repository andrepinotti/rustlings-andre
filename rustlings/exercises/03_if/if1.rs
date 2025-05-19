fn bigger(a: i32, b: i32) -> i32 {
    // DONE: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    let a = a;
    let b = b;

    if a > b{
        a
    } else if b > a{
        b
    } else {
        b // How i can return any number, i put as return the variable b
    }
}

fn main() {
    // You can optionally experiment here.
   let bigger = bigger(33, 43);
   println!("the bigger is {bigger}");
   
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
