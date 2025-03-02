pub fn learnPrimitives() {
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; //suffix annotation

    // Or a default will be used.
    let default_float = 3.0; //`f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred form another line
    inferred_type = 439467296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32
    mutable = 21; // Mutable ``i32`

    // Error! The type of a variable cnn't be changed.
    //mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    /* Compound type : Array and Tuple */
    // Array signature consists of type T and length as [T; length]
    // let my_array: [i32; 5] = [1, 2, 3, 3, 4, 5];

    //Tuble is a collection of values of different types and is cnstructed using parentheses () and commas ,
    // let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}

pub fn literalOperators() {
    // Integer additin
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer substration
    println!("1 -2 = {}", 132 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    //Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    //Short-circuiting boolean logic
    println!("true AND is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    //Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("00111 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    //Use underscores to improve readability
    println!("one million is written as {}", 1_000_000u32);
}

pub fn tubples() {}

pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    //let can be used to bind the membres of a tuple to variables
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

pub fn slice() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("First element of the array: {}", xs[0]);
    println!("The second element of the array : {}", xs[1]);

    // `len` returns the size of the array
    println!("Number of elements in the array : {}", xs.len());
    //arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&xs));

    //Arrays can be automatically borrowed as slices
    println!("Borow the whole array as a slice");
    analyze_slice(&xs);

    //Slices can point to a section of an array
    //They are of the form [starting_index..ending_index]
    //starting_index is the first position in the slice
    //ending_index is one more than the last position in the slice
    println!("Borrow a section of the array as a slice");
    analyze_slice(&ys[1..10]);

    // Example of empty slice
    let empty_slice: [i32; 0] = [];
    assert_eq!(&empty_slice, &[]);
    assert_eq!(&empty_slice, &[][..]);

    //Array can be safely accessed using `.get()` method, which retturns an `Option`. This can bee matched as shown below or used with
    //`.epxect()`, which will panic if the index is out of bounds
    //If you would like to the program to exit  with a nice message instead of  a panic, you might prefer `.unwrap_or_else()`
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}, {}", i, xval),
            None => println!("Slow down {} is too far!", i),
        }
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice : {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn custom_types() {
    #![allow(dead_code)]

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // a unit struct
    struct Unit;

    // a tuple struct
    struct Pair(i32, f32);

    // a struct with two fields
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // structs can be reused as fields of another struct
    #[derive(Debug)]
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }

    //create struct with field init shorthand
    let name = String::from("Shaheen");
    let age = 40;
    let shaheen = Person { name, age };
    let asmany = Person {
        name: String::from("Asmany"),
        age: 35,
    };
    //print debug struct
    println!("{:?}", shaheen);
    println!("{:?}", asmany);

    //Instantiate a Point
    let point = Point { x: 12.6, y: 12.23 };
    let another_point = Point { x: 3.2, y: 2.3 };
    println!(
        "Point coordinates: ({}, {})",
        another_point.x, another_point.y
    );
    //access the fields of points
    println!("Point coordinamtes : ({}, {})", point.x, point.y);

    //Make a new point by using struct update syntax to use the fields of our other point
    let bottom_point = Point {
        x: 10.3,
        ..another_point
    };
    println!(
        "Bootom point corrdinates : ({}, {})",
        bottom_point.x, bottom_point.y
    );

    //Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_point,
    };
    println!("Rectangel : {:?}", rectangle);

    //Instantiate a unit struct
    let _unit = Unit;
    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(num1, num2) = pair;
    println!("Pair contains {:?} and {:?}", num1, num2);
}

pub enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    //like tuple structs
    KeyPress(char),
    Paste(String),
    //or c-like structures
    Click { x: i64, y: i64 },
}
pub fn enum_a(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}.", x, y);
        }
    }
}

pub fn show() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my_text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    enum_a(pressed);
    enum_a(pasted);
    enum_a(click);
    enum_a(load);
    enum_a(unload);
}
