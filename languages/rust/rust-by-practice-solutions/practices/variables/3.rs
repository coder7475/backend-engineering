// * Scope: A scope is the range within the program for which the item is valid
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;

    {
        // z is valid within this scope, not outside
        let z: i32 = 20;
        println!("The value of x is {} and value of y is {} and z is {z}", x, y);
    }

    println!("The value of x is {} amd value of y is {}",x, y);
}
