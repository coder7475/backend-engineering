// Size of unit type
use std::mem::size_of_val;

fn main() {
  let v = ();

  println!("{}", size_of_val(&v)); // 0

  println!("Success!");
}