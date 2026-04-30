fn main(){
    
    /*
    * Solution 1
    let x = String::from("Hello World");
    let y = x.clone();
    */

    /*
    * Solution 2
    let x = "hello world";
    let y = x;
    */

    /*
    * Solution 3
    let x = &String::from("hello, world");
    let y = x;
    */

    /*
    * Solution 4
    
    */

    let x = &String::from("hello, world");
    let y = x.as_str();

    println!("{}, {}", x, y);
}