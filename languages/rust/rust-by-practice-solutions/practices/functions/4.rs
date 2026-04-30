
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
/* 
fn never_return_fn() -> ! {
    unimplemented!()
    } 
*/
    
// First Solution
// IMPLEMENT this function in THREE ways
/* 
fn never_return_fn() -> ! {
    panic!()
} 
*/

// Second Solution
// IMPLEMENT this function in THREE ways
/* 
fn never_return_fn() -> ! {
    todo!();
}
*/

// Third Solution
// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}


