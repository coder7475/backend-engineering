// Allow unused code in this module
#![allow(unused)]

fn main() {
    // Create an array of integers
    let a = [1, 2, 3, 4, 5];

    // Create a slice referencing elements from index 1 to 3 (exclusive)
    let slice = &a[1..3];

    // Print the slice using debug formatting
    println!("My Slice: {:?}", slice);

    // Verify the slice contains [2, 3]
    assert_eq!(slice, &[2, 3]);

    // Print success message
    println!("Success!");
}