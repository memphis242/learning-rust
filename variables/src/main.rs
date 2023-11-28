fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;  // Will cause an error since x is immutable by default!
    println!("The value of x is: {x}");
}
