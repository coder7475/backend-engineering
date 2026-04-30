// * Tips: If we don't explicitly assign a type to a variable,
// * then the compiler will infer one for us
fn main() {
    let x: i32 = 5;
    let mut y = 5; //? i32 - default integer type
   
    
    println!("first y is {y}");

    y = x;

    println!("second y is {y}");

    let _z = 10; //? i32

    println!("Success!");
}
