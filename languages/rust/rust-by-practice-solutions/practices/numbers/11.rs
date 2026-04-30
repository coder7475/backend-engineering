#[allow(unused_variables)]
fn main() {
    // * Integer addition
    assert!(1u32 + 2 == 3);

    // * Integer subtraction
    assert!(1i32 - 2i32 == -1);
    assert!(1i8 - 2 == -1);

    // * multiplication
    assert!(3 * 50 == 150);

    // * division
    assert!(9.6 as f32 / 3.2 as f32 == 3.0);

    // * modulus operation
    assert!(24 % 5 == 4);

    // * Short circuting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // * Bitwise operations

    println!("Success!")
}