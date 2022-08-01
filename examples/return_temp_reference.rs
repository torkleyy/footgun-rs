#![deny(unsafe_code)] // for extra safety

use footgun_rs::footgun;

fn return_temp_reference(input: &[i32]) -> &[i32] {
    let s = vec![input[2], input[1], input[0]];

    // &s <- this would error because the Rust compiler is boring
    footgun(&s) // <- compiles perfectly fine
}

fn main() {
    println!("reversed: {:?}", return_temp_reference(&[1, 2, 3]));
}
