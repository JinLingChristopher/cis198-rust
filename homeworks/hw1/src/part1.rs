/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

// n is not changefeed
pub fn double_v1(n: i32) -> i32 {
    return n * 2;
}

// n is a pointer to i32, it's content does not changed
pub fn double_v2(n: &i32) -> i32 {
    *n * 2
}

// n is a mutable pointer to i32, change it's content
pub fn double_v3(n: &mut i32) {
    // double n in place
    *n *= 2
}

// Example unit test (so you can recall the syntax)
#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
    assert_eq!(double_v1(0), 0);
}

#[test]
fn test_double_v2() {
    let a = 3;
    assert_eq!(double_v2(&a), 6);

    let a = -1;
    assert_eq!(double_v2(&a), -2);

    let a = 0;
    assert_eq!(double_v2(&a), 0);
}

#[test]
fn test_double_v3() {
    let mut a = 3;
    double_v3(&mut a);
    assert_eq!(a, 6);

    a = 0;
    double_v3(&mut a);
    assert_eq!(a, 0);

    a = -1;
    double_v3(&mut a);
    assert_eq!(a, -2);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    let mut result = 0;
    while result * result <= n {
        result += 1
    }
    result - 1
}

#[test]
fn test_sqrt() {
    assert_eq!(sqrt(0), 0);
    assert_eq!(sqrt(1), 1);
    assert_eq!(sqrt(2), 1);
    assert_eq!(sqrt(3), 1);
    assert_eq!(sqrt(4), 2);
    assert_eq!(sqrt(7), 2);
    assert_eq!(sqrt(9), 3);
}

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    let mut result = 0;
    for &v in slice {
        result += v;
    }
    result
}

#[test]
fn test_sum_v1() {
    let numbers = [1, 2, 3, 4, 5];
    assert_eq!(sum_v1(&numbers), 15);

    let a = [0; 0];
    assert_eq!(sum_v1(&a), 0);
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    let mut result = 0;
    for v in slice {
        result += *v
    }
    result
}

#[test]
fn test_sum_v2() {
    let a = [0; 0];
    assert_eq!(sum_v2(&a), 0);

    let b = [1, 2, 3, 4, 5];
    assert_eq!(sum_v2(&b), 15);
}

/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut result:Vec<i32> = Vec::new();
    for v in slice {
        let mut find = false;
        for item in &result {
            if *v == *item {
                find = true;
                break
            }
        }
        if !find {
            result.push(*v);
        }
    }
    result
}

#[test]
fn test_unique() {
    let a = [1, 2, 3, 1, 2, 3];
    assert_eq!(unique(&a), [1, 2, 3]);
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut result = Vec::new();
    for v in slice {
        if pred(*v) {
            result.push(*v)
        }
    }
    result
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    if out_size == 0 {
        return vec![];
    }
    if out_size == 1 {
        return vec![n1];
    }

    let mut result = vec![n1, n2];
    for i in 2..out_size {
        let a= result[result.len() - 1] + result[result.len() - 2];
        result.push(a);
    }
    result
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(0, 1, 0), []);
    assert_eq!(fibonacci(0, 1, 1), [0]);
    assert_eq!(fibonacci(0, 1, 2), [0, 1]);
    assert_eq!(fibonacci(0, 1, 3), [0, 1, 1]);
    assert_eq!(fibonacci(0, 1, 4), [0, 1, 1, 2]);
    assert_eq!(fibonacci(0, 1, 5), [0, 1, 1, 2, 3]);
    assert_eq!(fibonacci(0, 1, 6), [0, 1, 1, 2, 3, 5]);

    assert_eq!(fibonacci(2, 3, 6), [2, 3, 5, 8, 13, 21]);
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

#[test]
fn test_str_concat() {
    let a = "hello ";
    let b = "world";

    assert_eq!(str_concat(a, b), "hello world".to_string());
}

pub fn string_concat(s1: String, s2: String) -> String {
    s1 + &s2
}

#[test]
fn test_string_concat() {
    let a = "hello ".to_string();
    let b = "world".to_string();

    assert_eq!(string_concat(a, b), "hello world".to_string());
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut result = "".to_string();
    for s in v {
        result = string_concat(result, s)
    }
    result
}

#[test]
fn test_concat_all() {
    let words = vec![
        "this ".to_string(),
        "is ".to_string(),
        "21th ".to_string(),
        "century".to_string()];
    assert_eq!(concat_all(words), "this is 21th century");
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut result = Vec::new();
    for s in v {
        let number = s.parse().expect("s is not a valid number");
        result.push(number);
    }
    result
}

#[test]
fn test_parse_all() {
    let data = vec!["1".to_string(), "3".to_string()];
    assert_eq!(parse_all(data), [1, 3]);
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut result = Vec::new();
    for item in v {
        result.push(item.to_string());
    }
    result
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    if n = 9, 1, 1, 2, 3, 5, 8, 13, 21, 34

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonacci(n: usize) -> String {
    let fibs = fibonacci(1, 1, n);
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    let even_fibs = filter(&fibs, is_even);

    concat_all(print_all(even_fibs))
}

#[test]
fn test_concat_even_fibonacci() {
    assert_eq!(&concat_even_fibonacci(6), "28");
    assert_eq!(&concat_even_fibonacci(9), "2834");
}
