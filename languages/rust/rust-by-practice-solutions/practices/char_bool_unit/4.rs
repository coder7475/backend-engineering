fn main() {
  let f = false;
  let t = true && false;

  assert_eq!(f, t);
  
  println!("Success!", );
}