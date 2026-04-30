#[allow(unused_variables)]
#[allow(unused_assignments)]
// learn shadowing
fn main() {
    let mut x: i32 = 1;
    x = 7; // 7
           // Shadowing and rebinding
    let mut x = x; // 7
    x += 3; // 10
    println!("x is {x}");

    let y = 4;
    // Shadowing
    let y = "I can also bount to the text!";
    println!("Success");
}
