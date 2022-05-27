use std::fs;

pub fn get(year: i32, day: i32) -> String {
    let s: String = fs::read_to_string(format!("src/data/y{}/d{}/input.txt", year, day))
        .expect("Read string fails.");
    s
}
