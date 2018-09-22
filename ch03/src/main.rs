fn main() {
    // In this case, we can't assign a new value to `x`.
    // let x = 5;
    // println!("The value of x is: {}", x);

    // x = 6;
    // println!("The value of x is: {}", x);

    // However, in this case, we can, because of the `mut` prefix.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // In Rust, constants must have type annotations.
    // They can never be mutated, and they cannot be computed.
    const MY_CONST: u32 = 100_000;
    println!("The value of MY_CONST is: {}", MY_CONST);

    // Shadowing is possible in Rust.
    let x = 5;
    let x = x + 1;
    let x = x + 2;

    // prints: The value of x is: 8
    println!("The value of x is: {}", x);

    // Shadowing is simply re-use of the identifier. You can even change its type.
    // string
    let spaces = "   ";
    // integer
    let spaces = spaces.len();
    println!("We got {} spaces.", spaces);
}
