use std::collections::HashMap;
use crate::garden::vegetables::Asparagus;
use std::fs::File;
use std::io::ErrorKind;
// use rand::Rng;
// use std::{self, cmp::Ordering, io};
// use std::collections::*;
// use std::io::Result as IoResult;

pub mod garden;

// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    // let mut map = HashMap::new();
    // map.insert(1, 2);
    //
    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // // Vector initialization
    // // 1)
    // let v: Vec<i32> = Vec::new();
    // // 2)
    // let v = vec![1, 2, 3];
    //
    // // Vector updating(not allow type cast)
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    //
    // // Vector Reading Elements(Attention: get return Option)
    // let v = vec![1, 2, 3, 4, 5];
    //
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    //
    // // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }
    //
    // // Vector Iterating
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    //
    // // Using an Enum to Store Multiple Types
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // String
    // let data = "initial contents";
    //
    // let s = data.to_string();
    //
    // // the method also works on a literal directly:
    // let s = "initial contents".to_string();
    //
    // let s = String::from("initial contents");
    //
    // // Update string
    // let mut s1 = String::from("foo");
    // let s2 = "bar"; // s2 is &str
    // s1.push_str(s2);
    // println!("s2 is {s2}"); // s2 is bar
    //
    // let s1 = String::from("Hello, ");
    // let mut s2 = String::from("world!");
    // // note s1 has been moved here and can no longer be used
    // // s2 here is a deep-copy
    // let s3 = s1 + &s2;
    // println!("{s2}"); // correct s2 is still valid.
    // s2.push_str("...");
    // println!("{s2}"); // world!...
    // println!("{s3}"); // not change world!...
    //
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    //
    // // format will not remove any ones ownership
    // let s = format!("{s1}-{s2}-{s3}");
    // println!("{s}");
    // println!("{s1}"); // valid
    //
    // // Index of String:
    // // Only allow reference
    // // looked as bytes array
    // // Only allow slice
    // let s1 = String::from("hello");
    // let h = &s1[0..1];
    // println!("{h}");
    //
    // // Methods for Iterating Over Strings
    // for c in "Зд".chars() {
    //     println!("{c}");
    // } // 3 д
    //
    // for b in "Зд".bytes() {
    //     println!("{b}");
    // } // 208 151 208 180

    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // let team_name = String::from("Blue");
    // // Optional skilled, get return a Option object
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    //
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }
    //
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    //
    // let mut map = HashMap::new();
    // map.insert(&field_name, &field_value);
    //
    // // Overwrite and insert
    // // Option-like operation
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    //
    // println!("{:?}", scores);
    //
    // let text = "hello world wonderful world";
    //
    // let mut map = HashMap::new();
    //
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    //
    // println!("{:?}", map);

    let greeting_file_result =File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
