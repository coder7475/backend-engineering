// * First Solution
/* fn main()  {
  never_return();
  // make it unreachable
  println!("Failed!");
}

fn never_return()  {
  panic!("Thread Panicked");
} */

// * Second Solution
fn main() {
  never_return();
}

use std::thread;
use std::time;

fn never_return() -> ! {
  // implement this function, don't modify fn signatures
  loop {
      println!("I return nothing");
      // sleeping for 1 second to avoid exhausting the cpu resource
      thread::sleep(time::Duration::from_secs(1))
  }
}