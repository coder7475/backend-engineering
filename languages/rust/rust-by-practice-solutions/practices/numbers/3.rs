fn main() {
 let x = 5; // ? i32
//*  println!("{}", type_of(&x));
 assert_eq!("i32".to_string(), type_of(&x));
 println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}