fn main() {
    let x = 5;
    let p = &x;

    println!("The memory address of x is: {:p}, value of p is {}", p, x);
}
