use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env::args;

fn main() {
    if args().count() != 3 {
        panic!("you did it wrong");
    }

    let line_number = search_file(args().nth(1).unwrap().as_str(), args().nth(2).unwrap().as_str());
    match line_number {
        Some(num) => println!("{}", num),
        None => println!("not found"),
    }
}


fn search_file(file_path: &str, query: &str) -> Option<i32> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut index = 0;
    for line in reader.lines() {
        index += 1;
        if line.unwrap().contains(query) {
            return Some(index);
        }
    }

    None
}

#[test]
fn search_string_with_result() {
    let file_path = "test/fixtures/hello.txt";
    let query = "three";
    let line_number = search_file(file_path, query);
    assert_eq!(line_number, Some(3));
}

#[test]
fn search_string_without_result() {
    let file_path = "test/fixtures/hello.txt";
    let query = "seven";
    let line_number = search_file(file_path, query);
    assert_eq!(line_number, None);
}
