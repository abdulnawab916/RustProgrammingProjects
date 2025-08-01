fn main() {
    let x = plus_one(5);
    println!("The value of x is: {x}"); 
    
    // for loop w/ range operator and utilizing the .rev()
    // method
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    
    // 'String': a mutable string type which lives on heap memory
    let this_string = String::from("A heap based string");
    println!("{this_string}");
}

// basic incrementer function
fn plus_one(x: i32) -> i32 {
    x + 1
}

/*
    // NOTES - Day 1:
    // for loop w/ the `rev` method
    // rev():
        // Reverses an iterator’s direction.
        // Usually, iterators iterate from left to right. 
        // After using rev(), an iterator will instead iterate 
        // from right to left.
        // This is only possible if the iterator has an end, so rev() 
        // only works on DoubleEndedIterators.
    // Iterables in Rust:
        // ... 
    // '..': range operator
        // a..b denotes an exclusive range, does not 
        // include 'b'
        // a..=b denotes inclusive range, includes 'b' 
 */