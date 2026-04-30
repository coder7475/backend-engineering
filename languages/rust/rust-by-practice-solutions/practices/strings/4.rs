// This program demonstrates different ways to modify a String

fn main() {
    // Create a new mutable String containing "hello"
    let mut s = String::from("hello");
    // Add a single character ',' using push()
    s.push(',');
    // Add a string slice " world" using push_str()
    s.push_str(" world");
    // Use += operator to append "!" to the string
    s += "!";

    // Print the final string
    println!("{}", s);
}