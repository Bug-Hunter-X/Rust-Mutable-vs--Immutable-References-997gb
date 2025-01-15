fn main() {
    let mut x = 5;
    { // Creating a separate scope to limit the lifetime of the mutable reference
        let y = &mut x; 
        *y += 1;
    }
    let z = &x;  // z is immutable, but x has already been modified in the inner scope
    println!("x = {}", x); // this will print 6
    println!("z = {}", *z); // this will print 6 as well.
} 