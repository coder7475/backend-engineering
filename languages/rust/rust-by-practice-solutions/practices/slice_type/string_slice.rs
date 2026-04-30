/* 
 Write a function that takes a string of words separated by spaces and 
 returns the first word it finds in that string. If the function doesn't 
 find a space in the string, the whole string must be one word, so the 
 entire string should be returned.
*/ 

/**
 * ? Finds and returns the first word in a string
 * * @param s - The input string to search through
 * * @returns The index position where the first word ends (at the first space),
 *  
 */ 
 
fn first_word(s: &str) -> &str { // Function that takes a string reference and returns a string slice
    let bytes = s.as_bytes(); // Convert string to bytes for character-by-character processing

    for (i, &item) in bytes.iter().enumerate() { // Iterate through bytes with their indices
        if item == b' ' { // If we find a space character
            return &s[..i];  // Return slice from start up to space
        }
    }
    // If no space found, return slice of entire string
    &s[..]
}

fn main(){ 
    let mut s = String::from("Hello World!"); // Create mutable string to demonstrate slicing
    let _hello = &s[0..5]; // Slice containing "Hello"
    let _world = &s[6..11]; // Slice containing "World"
    let s1 = &s[..5]; // Another way to slice "Hello" (from start to position 5)
    let s2 = &s[6..]; // Slice from position 6 to end
    
    println!("s: {}, slice 1: {}, slice 2: {}", s, s1, s2); // Print full string and two slices
    
    let word = first_word(&s); // Get first word as a slice
    println!("the first word is: {word}");

    s.clear(); // Empty the string

    // ! println!("the first word is: {word}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}