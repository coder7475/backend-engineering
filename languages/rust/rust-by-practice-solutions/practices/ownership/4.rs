// Fix the error without removing any code
// * Sol 1
// fn main() {
//     let s = String::from("Hello World");

//     print_str(&s);

//     println!("{}", s);
// }

// fn print_str(s: &String)  {
//     println!("{}",s)
// }

// * Sol 2
// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}