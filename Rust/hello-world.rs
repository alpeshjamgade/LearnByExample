//! doc comments
fn main() {

    // println : prints arguments to the console.
    println!("Hello World!");
    println!("I'm Rustacean!");

    /*
     * Multiline Comment
     * Same as C/C++.
     * Rust is up!.
     */
    
    let x = 5 + 5;
    println!("x is {}", x);

    // {} will be replaced by any arguments.
    println!("{} days", 29);

    // Positional arguments can be used. Specify the integer inside `{}`
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");
    
    // DIfferent formatting can be invoked by specifying the format character after a `:`.
    println!("Base 10:                {}", 69420);
    println!("Base 2:                 {:b}", 69420);
    println!("Base 8:                 {:o}", 69420);
    println!("Base 16 (hexadecimal):  {:x}", 69420);
    println!("Base 16 (hexadecimal):  {:X}", 69420);


    // right or left justify text with specified width.
    println!("{number:>5}", number=1);
    
    // pad numbers with extra zeroes or any character.
    println!("{number:0>5}", number=1);
    println!("{number:x>5}", number=1);
    println!("{number:0<5}", number=1);
    
    // Name arguments iin the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=100);

    let number: f64 = 1.0;
    let width: usize = 5;

    return;
}
