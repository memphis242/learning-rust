fn main() {
    let x = 5;
    println!("The value of x at the beginning is: {x}");
    let x = x + 1;  // shadowing x
    println!("The value of x after the first shadow is: {x}");

    {
        let x = x * 2;  // shadowing x again
        println!("The value of x in the inner scope with the second shadow is: {x}");
    }

    println!("The value of x after the inner scope is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    let mut spaces = "    ";
    spaces = spaces.len();
    println!("spaces: {spaces}");
}
