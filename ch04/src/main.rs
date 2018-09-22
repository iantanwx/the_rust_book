fn main() {
    // Rust has signed and unsigned integers from width 8 to 128
    // it also has architecture-specific isize and usize, which are either 32 or 64 bits wide.
    // Rust is safe. If an integer overflows in `debug` build, the program will panic.

    // Rust has an interesting boolean type: it's basically an ADT.
    // They're 1 byte in size. They're consumed mainly in `if` or `match` expressions.
    let _t: bool = true;
    let _f: bool = false;

    // Rust supports utf codepoints up to U+10FFFF (32 bits)
    // `char`s can therefore be up to 4 bytes.
    let _emoji: char = '👬';

    // Rust supports tuples (yay!). It's a compound type.
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    // They can be destructured.
    let (x, y, z) = _tup;
    println!("The values x, y, z are: ({}, {}, {})", x, y, z);

    // Alternatively, the elements of a tuple can be accessed with dot notation.
    println!("Using dot notation: ({}, {}, {})", _tup.0, _tup.1, _tup.2);

    // Rust also has arrays; they're fixed-length.
    // Crucially, arrays are always allocated on the _stack_.
    // The type annotation is of the form [type; size].
    let my_array: [i8; 5] = [1, 2, 3, 4, 5];

    println!("Index {} has value {}", 3, my_array[3]);
}
