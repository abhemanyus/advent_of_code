use std::fs;

pub fn load_file(name: &str) -> String {
    let path = format!("data/{name}.txt");
    fs::read_to_string(path).unwrap()
}
