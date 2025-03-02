mod lessons;
use lessons::custom_types;
use lessons::literalOperators;
use lessons::reverse;
use lessons::show;
use lessons::slice;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    /* print!("{} days make a week.", 7);

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
    println!("{number:>width$}"); */
    // literalOperators();
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true, 12i32, 678i32,
        'z', false,
    );
    //values can be extraced from the tuple using tuple indexing
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);
    println!("Long tuple third value: {}", long_tuple.2);

    // Tuples can be tuple members
    let tuple_of_tuples = (1u8, 2u16, 2u32, (4u64, -1i8), -2i16);
    //tuples are printable
    println!("Tuple of tuples : {:?}", tuple_of_tuples);

    //But long tuples cannot be printed
    // println!("Long tuple: {:?}", long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse(pair));

    //To create one element tuple, the comma is required to tell them apart from a literal surrounded by parentheses
    println!("One element tuple: {:?}", (34u32,));
    println!("One elemnet tuple: {:?}", (456u32));

    //tuples can be destructured to create bindings
    let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) = long_tuple;
    println!("The value of a is : {}", a);
    println!("The value of b is : {}", b);
    println!("The value of m is : {}", m);

    let tuple = (1, "hello", 20.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, c);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:\n{:#?}", matrix);

    //Activity
    slice();
    println!("________________________custom_types : strucks__________________________________");
    custom_types();
    println!("________________________Enum__________________________________");
    show();
}
