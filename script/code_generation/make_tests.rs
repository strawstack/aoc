//use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn fill_test_template(year: i32, day: i32, part: i32, answer: i32) -> String {
    let test_template: String = format!("#[test]
fn y{year}_d{day}_p{part}() {{
    let s: &str = &data::get({year}, {day});
    assert_eq!(y{year}::d{day}::p{part}::main(s), {answer});
}}", year = year, day = day, part = part, answer = answer);
    test_template
}

type Year = i32;
type Day = i32;
type Part = i32;
type Answer = i32;

fn main() {

    let mut answers: HashMap<(Year, Day, Part), Answer> = HashMap::new();
    answers.insert((2015, 1, 1), 232);
    answers.insert((2015, 1, 2), 1783);
    answers.insert((2015, 2, 1), 1606483);
    answers.insert((2015, 2, 2), 3842356);
    answers.insert((2015, 3, 1), 2081);
    answers.insert((2015, 3, 2), 2341);
    answers.insert((2015, 4, 1), 254575);
    answers.insert((2015, 4, 2), 1038746);
    answers.insert((2015, 5, 1), 238);
    answers.insert((2015, 5, 2), 69);
    answers.insert((2015, 6, 1), 377891);
    answers.insert((2015, 6, 2), 14110788);
    answers.insert((2015, 7, 1), 3176);
    answers.insert((2015, 7, 2), 14710);

    for y in 2015..2016 {
        for d in 1..=25 {
            for p in 1..=2 {
                let entry = answers.entry((y, d, p));
                match entry {
                    Entry::Occupied(entry) => {
                        let s: &str = &fill_test_template(y, d, p, *entry.get());
                        println!("{}\n", s);
                    },
                    Entry::Vacant(_) => continue
                }
            }
        }
    }
}
