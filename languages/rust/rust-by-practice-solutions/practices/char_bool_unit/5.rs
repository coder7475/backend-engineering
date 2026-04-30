// Unit type = ()
fn main() {
  let v = ();

  assert_eq!(v, implicitly_ret_unit());

  println!("Success!");
}

fn implicitly_ret_unit() {
  println!("This returns unit type: ()");
}