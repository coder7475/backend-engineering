// Use two approaches to fix the error and without adding a new line
fn main() {
    // Sol 1
    // let s = "hello, world".to_string();
    // let s1: String = s;

    let s = String::from("hello, world");
    let s1: String = s;

    println!("Success!");
}