fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y += 1;       // Modifies x through y
    let z = &x;      // z is an immutable reference to x
    println!("x = {}", x); // this will print 6
    println!("z = {}", *z); // this will also print 6, however, ...
    // attempt to modify x through z will result in compiler error:
    //*z += 1; // error[E0594]: cannot assign to `*z` which is behind a `&` reference
}