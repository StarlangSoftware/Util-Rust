use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileUtils{
}

impl FileUtils{

    pub fn new() -> FileUtils{
        FileUtils{}
    }

    pub fn read_hash_map(file_name: &str) -> HashMap<String, String>{
        let mut result = HashMap::new();
        let file = File::open(file_name).expect("Unable to open input file");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            let items: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
            result.insert(items[0].clone(), items[1].clone());
        }
        result
    }
}