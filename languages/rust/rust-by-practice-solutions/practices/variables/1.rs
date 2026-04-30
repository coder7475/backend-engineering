/*
* Binding and Mutability
? 1. A variable can be used only if it has been initialized
*/
fn main() {
    let x:i32 = 5;
    // x = 88; // !error: can't reassign on immutable value x
    assert_eq!(x, 5);
    println!("Success!")
}
