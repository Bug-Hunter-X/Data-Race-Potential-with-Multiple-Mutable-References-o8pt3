fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;    // z is an immutable reference to x
    *y += 1;       // Modify x through y
    println!("x = {}", x); // x is now 6
    println!("z = {}", *z); // z is still 5, no change
    let w = &mut x;  // w is a mutable reference to x (this is problematic)
    *w += 1;        // Modify x through w
    println!("x = {}", x); // x is now 7
}