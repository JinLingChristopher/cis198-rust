// Classroom Minutia:
// Homework 1 is up! Do the first half of homework 1.
// Clippy and rust fmt.
// Rust train model.
// Not only to learn Rust, but understand the design behind Rust.

// Why does Rust do things this way?
// How does this related to other languages?
// What are the tradeoffs?

// Starting Things.
// Check out the simple cargo structure for a Rust project.

// `cargo new lecture2`

// Usual cargo Rust Project layout.
// Cargo.toml src/ examples/ tests/

// Check out the Cargo.toml file.

// In Rust we specify the types of integers.
// What are the tradeoffs of using different types?
// Peformance and memory size.
#[allow(dead_code, unused_variables)]
fn machine_types() {
    let n: i8 = 0;
    let m: i16 = 0;
    let o: i32 = 0;

    // Different size literals
    let p = 0i64;
    let p2 = 0 as i64;
    // Unsigned versions.
    let n = 0u32;

    // Machine dependent types
    let n: isize = 40;
    // Indexer!
    let i: usize = 40;

    let array: [i32; 3] = [1, 2, 3];

    // Why do we have specific sizes instead of the C/C++ approach?
    // Implicit sizes are not good enough for systems programming, where the size
    // in bytes matters.
    // https://stackoverflow.com/questions/11438794/is-the-size-of-c-int-2-bytes-or-4-bytes


    // Why doesn't C specify a size for integers?

    // Whats happens if we try to use a non usize for an array index?
    // let i: isize = 1;
    array[i /*as usize*/];

    // Why are array indices usize i.e machine dependent?
    // Same reason C doesn't specify size. To allow flexibility with the machine
    // dependent code.

    // Chars and bools are not ints!
    let b = true;
    let c = 'c';

    // if b /*as i32*/ == 1 {

    // }
    // Use as operator to convert to different types:

}

// Arrays and vectors.
#[allow(dead_code, unused_variables)]
fn array() {
    // Size is hardcoded for arrays.
    // There space is allocated directly in the binary.
    // Probably in the .data or .rodata sections of an ELF file.
    let a = [1, 2, 3];
    let zeroes = [0; 1000];
    let b: [i32; 3] = [1, 2, 3];

    // Instead, Vectors are dynamically allocated.
    // Talk about heap and stack, and "pointers".
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(3);

    // Rust vectors have their capacity and length fields
    // as part of the vector size, accessing elements requires
    // following the pointer down to the heap.

    // How is this different from Objects in other languages?
}

// Arrays, vectors, and slices.
#[allow(dead_code, unused_variables)]
fn slices() {
    let a = [1, 2, 3];
    // Fat pointer: pointer to heap, length, capacity,
    let v = vec![1, 2, 3];

    // Take a reference to the address of the vector.
    // We call this a _slice_, notice the type has changed to
    // resemble an array.

    // Arrays and Vector references kinda have the same memory layout...
    let sv: &[i32] = &v;
    let av: &[i32] = &a;
    // So we can take references to both! (There is some automatic conversion from
    // &Vec<i32> to &[i32])...

    // How big are references?
    // Probably the size a pointer.

    // Accept a vector:
    // Write functions that can take either.
    fn print(s: &[i32]){
        unimplemented!()
    }

    // Same function prints both arrays and vector references.
    print(sv);
    print(av);
    // Rust knows how to convert vectors to arrays.
    print(&v);
    print(&v[0..1]);
}

#[allow(dead_code, unused_variables)]
fn strings() {
    // The problem with C style char*:
    // Not obvious if it's modifyiable.

    // Where does this reference live
    // .data
    let s: &str = "rust";
    // ^ inmutable.

    // Rust seems to have several types of strings: str, &str, and String.
    // Why do we do this? People don't like this about rust.
    // But like everything else, Rust has good reason!

    // Heap allocated, string.
    // UTF-8, growable, modifyiable, guaranteed to be correct UTF-8
    let s: String = "rust".to_string();

    // Example in book
    let noodles: String = "noodles".to_string();
    let oodles: &str = &noodles[1..];
    let poodles = "ಠ_ಠ"; // Pre allocated read-only memory.

    fn print(s: &str) {

    }

    // Notice `str` by _itself_ is a type! What does this mean?
    let x: Box<str> = noodles.into_boxed_str();

    // Unsized types.
    // Why don't languages like Java or Python have this issue?

    // String are laid out in pretty much the same way as arrays.
    // Notice the analogies between strings and &str:
    // String ~~ Vec<T>
    // &str ~~ &[T]
    // str ~~ [T]
}

#[allow(dead_code, unused_variables)]
fn structured_types() {
    // Tuple
    // How are tuples different than arrays?
    // - Heterogenous Data
    // Number of elements is inmutable.
    let t = (5, "cat");
    // Tuple Accessor
    let num = t.0;
    let str1 = t.1;

    // int pipe(int pipefd[2]);
    // pub fn pipe() -> Result<(RawFd, RawFd)>

    // Struct (More on structs later)
    struct Coordinate {
        x: f32,
        y: f32,
    };

    // Enums (more on enums later)
    enum Weekdays {
        Monday,
        Tuesday,
        Wednesday,
    }
}

#[allow(dead_code, unused_variables)]
fn references() {
    // Think of them as C/C++ pointers, but safe!
    // Always refer to valid data.

    // A reference lives in the stack of the current function, but
    // may point to data in stacks "above" or somewhere in the address space.
    let x = 3;
    let r = &x;
    let y = &3;

    // First version.
    // fn add1(r: &i32) {
    //     r += 1;
    // }

    // Second version
    // fn add1(r: &i32) {
    //     *r += 1;
    // }

    // Working version
    // fn add1(r: &mut i32) {
    //     *r += 1;
    // }

    // add1(r);
}

// Heap allocations.
#[allow(dead_code, unused_variables)]
fn boxes() {
    // Rust puts structures in the stack. No dereference needed!
    let t = (12, "eggs");

    // VS Java and Python which allocate all non-primitives on the heap.
    // This incurs a pointer dereference to access all data.

    // When `b` goes of scope, memory is freed (unless moved)
    let b = Box::new(t);

    // Raw pointers.
    let x = 3;
    let raw: *const i32 = &3;
}

#[allow(dead_code, unused_variables)]
fn unit_type() -> () {
    //  What is the return type of the print function?
    let r: () = println!("hello");

    // Type of this function?
    return ();
}

// ifs are expressions, not statements.
// What is the difference between an expression and statement?
fn if_expr(b : bool) {
    let x: i32 =
        if b {
            5
        } else {
            10
        };
}


fn while_loop(b: bool) {
    while b {
        unimplemented!()
    }

    loop {
        // Do things.
    }
}

fn for_loop() {
    // Using ranges, exclusive
    for i in 0..20 {
        println!("{}", i);
    }

    // Inclusive
    for i in 0..=20 {
        println!("{}", i);
    }

    let v: Vec<&str> = vec!["cis", "198", "rust", "programming"];

    // Do not iterate over i with a index. Instead use iterators!
    for e in v {
        let e2: &str = e;
        // Do stuff with v
    }

    // What are the problems with indexing arrays with [i]?
    // Must (should) do bounds checking for array.
    // Instead the compiler is able to omit bounds checking, as it generates
    // the iteration code itself.
}

fn function_calls(v: Vec<i32>) {
    // Types can have methods:
    let v2 = v.clone();
    // Or associated functions.
    let mut v3: Vec<i32> = Vec::new();
    v3.push(4);

    // If compiler cannot infer the type of a generic function, turbofish
    // syntax may be required:
    // turbo.fish
    let v4 = Vec::<i32>::new();

    // Lets see what documentation looks like:
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html
    use std::collections::HashMap;
}

// Initial code. Showing all explicit types.
fn build_vector_rough() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

// Question: Can we omit the type of the function? No

// Omiting types:
#[allow(dead_code)]
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

// Final version.
#[allow(dead_code)]
fn build_vector_final() -> Vec<i16> {
    vec![10, 20]
}

// NOTE: Will NOT talk about in class.
fn reference_dereference(s: &mut String) {

    fn f(i: i32){
        unimplemented!()
    }

    let v = vec![0; 10];
    for i in &v {
        f(*i);
    }
    s.push('c');
}

fn closure() {
    // Anonymous functions, capture enviornment.
    let x = 3;
    let f = |y| y + 3;
    assert_eq!(f(5), 8);

    let is_even = |x: i32| -> bool {x % 2 == 0};

    // Type inference.
    let vec : Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec2 = vec.into_iter().filter(|x| x % 2 == 0).collect::<Vec<i32>>();

    // let ht: HashMap<i32, i32> = v.clone().into_iter().map(|x| (x, x)).collect();
}
