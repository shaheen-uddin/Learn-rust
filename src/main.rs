fn main() {
    print!("{} days make a week.", 7);

    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Shaheen");
    println!(
        "{subject} {verb} {object}",
        object = "you",
        subject = "I",
        verb = "love"
    );

    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);

    println!("{number:0<width$}", number = 1, width = 8);
    println!("My name is {0}, {1}", "Shaheen", "Uddin");

    // #[allow(dead_code)]
    // struct Structure(i32);
    //  println!("This struct `{}` won't print...", Structure(3));
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
