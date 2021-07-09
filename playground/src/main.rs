use std::collections::HashMap;

fn variable() {
    // declare a variable
    let number;

    let actor = "Robert Denero";

    number = 233;

    println!("The number is {}", number);
    println!("The actor is {}", actor);

    // variable shadow, new number will shadow old number variable
    let number = 5;
    let number = number + 5;
    let number = number * 2;

    println! ("the number is {}", number);
}

fn mutable_variable() {
    let mut number = 10;
    println! ("the number is {}", number);

    number = 233;
    println! ("the number is {}", number);
}

fn data_type() {
    let number: u32 = 233;
    println! ("the number is {}", number);

    let float: f64 = 3.1415;
    println!("the float number is {}", float);

    println!("1 + 2 = {}, 8 - 5 = {}, 15 * 3 = {}", 1u32 + 2, 8 - 5u64, 15u32 * 3u32);
    println!("9 / 2 = {}, 9.0 / 2.0 = {}", 9 / 2, 9.0 / 2.0);

    // bool
    let greater = 1 > 2;
    println!("is 1 greater than 2 ? => {}", greater);

    // char
    let uppercase = 'A';
    let lowercase = 'c';
    let emoji = '😃';

    // str, also as `character slice`, use `&str` as pointer to an immutable string data
    // it can be used at compile time, if we now the whole content of the str
    let string_1 = "hello";
    let string_2: &str = "world";

    // String, allocate on heap, should be used if we want to get data from user input.
}

fn tuple_example() {
    // the size of tuple can not be changed
    let tuple = ("Edison", "Jin", 1994, 07, 28);
    println!("Name: {}; Family Name: {}, Birth: \"{}-{}-{}\"", tuple.0, tuple.1, tuple.2, tuple.3, tuple.4)
}

fn condition_if() {
    if 1 == 2 {
        println!("1 equal to 2")
    } else {
        println!("you are right, I fule you")
    }
}

struct Student {
    name: String,
    level: u8,
    pass: bool
}

struct Grades(char, char, char, char, f32);

struct Unit;

fn compond_data() {
    let user_a = Student {
        name: String::from("Constance Sharma"),
        pass: false,
        level: 2
    };

    let mark_a = Grades('A', 'A', 'A', 'B', 3.75);

    println!("student info: name = {}, level = {}, pass = {}, Grades = {}, {}, {}, {}, {}", user_a.name, user_a.level, user_a.pass, mark_a.0, mark_a.1, mark_a.2, mark_a.3, mark_a.4);
}

// define variant by struct
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
struct KeyPress(String, char);

// Define an enum with three variants
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
struct MouseClick{
    x : i64,
    y: i64
}

// Define an enum with three variants
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKey(KeyPress)
}

fn enum_example() {
    let load = WebEvent::WELoad(true);
    let click = WebEvent::WEClick(MouseClick{
        x: 100,
        y: 100
    });
    let press = WebEvent::WEKey(KeyPress(String::from("Ctrl+"), 'N'));

    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", load, click, press);

}

fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        println!("\nError! Division by zero is not allowed.");
        // To prevent division by zero, halt execution and return to the caller
        return false;
    } else if dividend % divisor > 0 {
        println!("\n{} % {} has a remainder of {}.", dividend, divisor, (dividend % divisor));
    } else {
        println!("\n{} % {} has no remainder.", dividend, divisor);
    }

    // Create the boolean value and return it to the function caller
    dividend % divisor == 0
}

fn is_zero(input: u8) -> bool {
    input == 0
}

fn array_example() {
    // array
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let bytes = [0; 5];

    // vec
    let vec_a = vec![3, 1, 4];
    println!("Initial vector: {:?}", vec_a);

    let zeros = vec![0, 5];
    println!("zeros: {:?}", zeros);

    // create empty vector, declare mutable so it can grow and shrink
    let mut fruit = Vec::new();
    fruit.push("apple");
    fruit.push("banana");
    fruit.push("cherry");
    println!("first is {}, last is {}", fruit[0], fruit[fruit.len() - 1]);
}

fn hash_example() {
    let mut reviews = HashMap::new();
    reviews.insert("Ancient Roman History".to_string(), "very accurate".to_string());
    reviews.insert("Cooking with Rhubarb".to_string(), "Sweet recipes".to_string());

    let book = "Cooking with Rhubarb";
    println!("Review for \'{}\': {:?}", book, reviews.get(book));
}

fn main() {
    variable();

    mutable_variable();

    data_type();

    tuple_example();

    condition_if();

    compond_data();

    enum_example();

    is_divisible_by(12, 4);
    is_divisible_by(13, 5);
    is_divisible_by(14, 0);

    is_zero(1);
    is_zero(0);
    is_zero(233);

    array_example();

    hash_example();
}
