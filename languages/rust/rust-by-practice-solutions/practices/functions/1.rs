fn main()  {
  let (x, y) = (1, 2);
  let s = sum(x, y);

  println!("sum = {}", s);
}

fn sum(x: i32, y: i32) -> i32 {
  x + y
}