use std::fs::File;
use std::io::{Read, Write};
use crate::distribution::Distribution;

const PATH: &str = "distributions.json";

pub fn read_from_file() -> String {
    let mut file = match File::open(PATH) {
        Ok(result) => result,
        Err(e) => panic!("Error opening file: {:?}", e)
    };

    let mut distributions_on_file = String::new();

    match file.read_to_string(&mut distributions_on_file) {
        Ok(_) => (),
        Err(e) => panic!("Error reading file content: {}", e)
    };

    distributions_on_file
}

pub fn write_to_file(distributions: &Vec<Distribution>) {
    let distributions_json = match serde_json::to_string_pretty(&distributions) {
        Ok(result) => result,
        Err(e) => panic!("Error mapping structs to json: {:?}", e)
    };

    let mut file_to_write = match File::create(PATH) {
        Ok(result) => result,
        Err(e) => panic!("Error opening/creating file: {}", e)
    };

    match file_to_write.write_all(distributions_json.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("Error writing to file: {:?}", e)
    }
}
