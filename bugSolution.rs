fn main() {
    let mut x = 5;
    { // New scope for y
        let y = &mut x; // y is a mutable reference to x
        *y += 1;       // Modify x through y
    }                // y goes out of scope
    println!("x = {}", x); // x is now 6
    let z = &x;    // z is an immutable reference to x
    println!("z = {}", *z); // z is 6
    { // New scope for w
        let w = &mut x;  // w is a mutable reference to x
        *w += 1;        // Modify x through w
    } // w goes out of scope
    println!("x = {}", x); // x is now 7
} 