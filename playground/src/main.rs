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

fn main() {
    variable();

    mutable_variable();

    data_type();

    tuple_example();

    condition_if();
}
