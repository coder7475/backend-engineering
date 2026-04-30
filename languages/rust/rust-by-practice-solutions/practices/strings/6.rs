// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; // We need to clone s1 since the + operator takes ownership of the first String
    assert_eq!(s3, "hello,world!"); // s3 concatenates s1 and s2
    println!("{}", s1); // We can still use s1 because we cloned it above
}