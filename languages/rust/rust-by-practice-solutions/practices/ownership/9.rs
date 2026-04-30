
fn main() {
    let t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     // * Sol 1
    //  ? let (ref s1, ref s2) = t;
    // * Sol 2
    let (s1, s2) = t.clone();
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }