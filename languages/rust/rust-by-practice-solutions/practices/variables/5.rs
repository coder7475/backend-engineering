// * Shadowing: Declare a new variable with the same name as previous variable
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        // within this scope x shadowed to 12 using let 
        assert_eq!(x, 12);

        println!("{}", x);
    }
    // Shadowing is over now x is 5
    assert_eq!(x, 5);
    println!("{}", x);

    // shadow again to 42
    let x: i32 = 42;
    println!("{}", x);
}
