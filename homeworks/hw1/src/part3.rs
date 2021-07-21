/*
    CIS198 Homework 1
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    The Rust borrow checker may help avoid some possible bugs.

    Then answer this question:
    Q: A common source of error in swap implementations is failing to work if
       the two references are the same. Why don't you need to worry about this
       case in Rust?
    A: cannot borrow a variable as mutable more than once at a time.

    (Try writing a unit test where they are both
    the same, i.e. swap_ints(&mut x, &mut x).)
*/
pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    let t = *x1;
    *x1 = *x2;
    *x2 = t;
}

#[test]
fn swap_ints_test() {
    let mut a = 1;
    let mut b = 2;
    swap_ints(&mut a, &mut b);

    assert_eq!(a, 2);
    assert_eq!(b, 1);
}
/*
    Problem 2: String duplication
*/
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?
// A: value borrowed here after move.

// Q2. How come it works fine here?
// A: i32 is allocated on stack
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let t = s.to_string();
    let mut result = Vec::new();
    for _ in 0..times {
        result.push(t.clone());
    }
    result
}

#[test]
fn duplicate_string_test() {
    let s = "chio";
    let obtained = duplicate_string(s, 3);
    let expected = vec!["chio", "chio", "chio"];

    assert_eq!(expected, obtained);
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: /* Change in here only*/ &String) -> String {
    string.clone()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(/* Change in here only*/ &str1));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1 /* Change in here only*/);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
// fn new_ref_string<'a>() -> &'a String {
//     let a = String::from("chio");
//     return &a;
// }
//
// #[test]
// fn test_new_ref_string() {
//     let obtained = new_ref_string().as_str();
//     assert_eq!(obtained, "chio".to_string().as_str())
// }
// A: not possible. a reference to a local variable cannot be returned.

fn  new_ref_str<'a>() -> &'a str {
    "chio"
}

#[test]
fn test_new_ref_str() {
    let a = new_ref_str();
    assert_eq!(a, "chio");
}

// The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    s2
}

#[test]
fn test_pick_longest2() {
    let s1 = "hello";
    let s2 = "world";
    assert_eq!(pick_longest2(s1, s2), "world");

    let s3 = "king";
    assert_eq!(pick_longest2(s1, s3), s1);
}

// A: we should not able to return reference from a function,
// since it's out of stack variable's lifetime.

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    If the vector is empty, return "".

    Q1. In pick_longest_in_v2, if you were to explicitly specify the lifetime
        of the input and output, what should it be?

    Q2. What are the pros and cons of v1 and v2?
*/

// result move each item into it in the loop, no real memory copy.
// rust make shallow copy and let previous invalidate, it's move.
fn pick_longest_in_v1(v: Vec<String>) -> String {
    let mut result = "".to_string();
    for item in v {
        if item.len() > result.len() {
            result = item
        }
    }
    return result
}

#[test]
fn test_pick_longest_in_v1() {
    let words = vec!["yu".to_string(), "xuan".to_string(), "chio".to_string(),
                     "pretty".to_string(), "gorgeous".to_string()];
    let result = pick_longest_in_v1(words);
    assert_eq!("gorgeous".to_string(), result);
}

// &str located in static memory area, result is a local variable on the stack
// it will copy item each loop, and then move to the caller function
fn pick_longest_in_v2(v: Vec<&str>) -> &str {
    let mut result = "";
    for item in v {
        if item.len() > result.len() {
            result = item
        }
    }

    result
}

#[test]
fn test_pick_longest_in_v2() {
    let words = vec!["This", "is", "a", "rust", "programming", "course"];
    let result = pick_longest_in_v2(words);
    assert_eq!("programming", result)
}

/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() if necessary to make any additional unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    let zeros = vec![0usize].repeat(desired_len - v.len());
    let result = [v, zeros].concat();
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    pad_with_zeros_v1(Vec::from(slice), desired_len)
    // debug_assert_eq!(result.len(), desired_len);
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    v.resize(desired_len, 0)
    // debug_assert_eq!(v.len(), desired_len);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row)
}

#[test]
fn test_append_row() {
    let grid = &mut vec![vec![true], vec![false]];
    append_row(grid, vec![true]);
    assert_eq!(grid, &mut vec![vec![true], vec![false], vec![true]]);
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    // Check if row is the first row in grid
    // Remember to handle the case when grid is empty
    match grid.first() {
        Some(r) => *r == row.to_vec(),
        None => false,
    }
}

#[test]
fn test_is_first_row() {
    let grid = vec![vec![true], vec![false]];
    assert_eq!(is_first_row(&grid, &vec![false]), false);
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::collections::HashMap;

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    let result = v.into_iter().cloned().collect();
    result
}

#[test]
fn test_vector_to_hashmap() {
    let pairs = vec![(1, "haha".to_string()), (2, "maybe".to_string())];
    let obtained = vector_to_hashmap(&pairs);

    let mut expected = HashMap::new();
    expected.insert(1, "haha".to_string());
    expected.insert(2, "maybe".to_string());

    assert_eq!(obtained, expected);
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    h.retain(|k, _| *k >= 0)
}

#[test]
fn test_delete_negative_keys() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(2, 3);

    let expected = map.clone();

    map.insert(-1, 0);

    delete_negative_keys(&mut map);
    assert_eq!(map, expected);
}

/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String,String>
) {
    for (k, v) in add.into_iter() {
        merged.entry(k).and_modify(|val| val.push_str(v.as_str())).or_insert(v);
    }
}

#[test]
fn test_merge_maps() {
    let mut merged: &mut HashMap<String, String> = &mut HashMap::new();
    merged.insert("a".to_string(), "1".to_string());

    let mut add: HashMap<String, String> = HashMap::new();
    add.insert("b".to_string(), "2".to_string());
    add.insert("a".to_string(), "3".to_string());

    merge_maps(&mut merged, add);

    let mut expected: HashMap<String, String> = HashMap::new();
    expected.insert("a".to_string(), "13".to_string());
    expected.insert("b".to_string(), "2".to_string());

    for (key, val) in expected.into_iter() {
        match merged.get(&key) {
            Some(merged_value) => assert_eq!(val, *merged_value),
            None => assert!(false),
        }
    }
}
