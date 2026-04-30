// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    // Solution 1
    greetings(s.to_string());
    // Solution 2
    greetings(String::from(s));
}

fn greetings(s: String) {
    println!("{}", s)
}