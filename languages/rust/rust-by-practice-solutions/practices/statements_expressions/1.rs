fn main() {
    let v = {
      let mut x = 1;
      x += 2; // statement
      x
    };

    assert_eq!(v, 3);

    println!("v: {}", v);
}
