// use std::collections::HashMap;
use crate::garden::vegetables::Asparagus;
// use rand::Rng;
// use std::{self, cmp::Ordering, io};
// use std::collections::*;
// use std::io::Result as IoResult;

pub mod garden;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    // let mut map = HashMap::new();
    // map.insert(1, 2);
    //
    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // Vector initialization
    // 1)
    let v: Vec<i32> = Vec::new();
    // 2)
    let v = vec![1, 2, 3];

    // Vector updating(not allow type cast)
    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    // Vector Reading Elements(Attention: get return Option)
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Vector Iterating
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
