use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let filename = get_file_name();
    let text = read_file(&filename);
    let result: HashMap<String, i64> = split_words(text);

    for (key, value) in result {
        println!("{} : {}", key, value);
    }
}

fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Wil need to pass filename");
            panic!();
        }
        2 => args[1].clone(),
        3 => {
            println!("Can accept only one filename");
            panic!();
        }
        _ => {
            println!("Something went wrong");
            panic!();
        }
    }
}

fn read_file(filename: &String) -> String {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            println!("There was an error in getting file {}", e.to_string());
            panic!()
        }
    };
    let mut buff_reader = BufReader::new(file);
    let mut contents = String::new();
    match buff_reader.read_to_string(&mut contents) {
        Ok(value) => value,
        Err(e) => {
            println!("There was an error in getting file {}", e.to_string());
            panic!()
        }
    };

    return contents;
}

fn split_words(text: String) -> HashMap<String, i64> {
    let mut map: HashMap<String, i64> = HashMap::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for word in words {
        if map.contains_key(word) {
            let orginal_value = match map.get(word) {
                Some(value) => *value,
                None => 0,
            };
            *map.entry(String::from(word)).or_insert(orginal_value) += 1;
        } else {
            map.insert(String::from(word), 1);
        }
    }
    return map;
}
