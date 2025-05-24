// DONE: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec_new_name = vec;

    vec_new_name.push(88);

    return vec_new_name;
}

fn main() {
    // You can optionally experiment here.

    let vec = Vec::new();

    let vec_example = fill_vec(vec);

    println!("My vec: {:?}", vec_example);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
