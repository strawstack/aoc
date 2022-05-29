//use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

type Year = i32;
type Day = i32;
type Part = i32;

#[derive(Copy, Clone)]
enum Ans<'a> {
    Number(i64),
    Str(&'a str),
}

fn fill_test_template(year: i32, day: i32, part: i32, answer: Ans) -> String {
    let test_template: String = format!("#[test]
fn y{year}_d{day}_p{part}() {{
    let s: &str = &data::get({year}, {day});
    assert_eq!(y{year}::d{day}::p{part}::main(s), {answer});
}}", 
    year = year, 
    day = day, 
    part = part, 
    answer = if let Ans::Number(ans) = answer { ans.to_string() } 
        else if let Ans::Str(ans) = answer { format!("\"{}\"", ans) }
        else { panic!("Answer type if let fails.") });
    test_template
}

fn main() {

    let mut answers: HashMap<(Year, Day, Part), Ans> = HashMap::new();
    answers.insert((2015, 1, 1), Ans::Number(232));
    answers.insert((2015, 1, 2), Ans::Number(1783));
    answers.insert((2015, 2, 1), Ans::Number(1606483));
    answers.insert((2015, 2, 2), Ans::Number(3842356));
    answers.insert((2015, 3, 1), Ans::Number(2081));
    answers.insert((2015, 3, 2), Ans::Number(2341));
    answers.insert((2015, 4, 1), Ans::Number(254575));
    answers.insert((2015, 4, 2), Ans::Number(1038736));
    answers.insert((2015, 5, 1), Ans::Number(238));
    answers.insert((2015, 5, 2), Ans::Number(69));
    answers.insert((2015, 6, 1), Ans::Number(377891));
    answers.insert((2015, 6, 2), Ans::Number(14110788));
    answers.insert((2015, 7, 1), Ans::Number(3176));
    answers.insert((2015, 7, 2), Ans::Number(14710));
    answers.insert((2015, 8, 1), Ans::Number(1371));
    answers.insert((2015, 8, 2), Ans::Number(2117));
    answers.insert((2015, 9, 1), Ans::Number(117));
    answers.insert((2015, 9, 2), Ans::Number(909));
    answers.insert((2015, 10, 1), Ans::Number(492982));
    answers.insert((2015, 10, 2), Ans::Number(6989950));
    answers.insert((2015, 11, 1), Ans::Str("vzbxxyzz"));
    answers.insert((2015, 11, 2), Ans::Str("vzcaabcc"));
    answers.insert((2015, 12, 1), Ans::Number(191164));
    answers.insert((2015, 12, 2), Ans::Number(87842));
    answers.insert((2015, 13, 1), Ans::Number(618));
    answers.insert((2015, 13, 2), Ans::Number(601));
    answers.insert((2015, 14, 1), Ans::Number(2640));
    answers.insert((2015, 14, 2), Ans::Number(1102));
    answers.insert((2015, 15, 1), Ans::Number(21367368));
    answers.insert((2015, 15, 2), Ans::Number(1766400));

    answers.insert((2016, 1, 1), Ans::Number(234));
    answers.insert((2016, 1, 2), Ans::Number(113));
    answers.insert((2016, 2, 1), Ans::Number(33444));
    answers.insert((2016, 2, 2), Ans::Str("446A6"));
    answers.insert((2016, 3, 1), Ans::Number(982));
    answers.insert((2016, 3, 2), Ans::Number(1826));
    answers.insert((2016, 4, 1), Ans::Number(173787));
    answers.insert((2016, 4, 2), Ans::Number(548));
    answers.insert((2016, 5, 1), Ans::Str("f97c354d"));
    answers.insert((2016, 5, 2), Ans::Str("863dde27"));
    answers.insert((2016, 6, 1), Ans::Str("usccerug"));
    answers.insert((2016, 6, 2), Ans::Str("cnvvtafc"));
    answers.insert((2016, 7, 1), Ans::Number(118));
    answers.insert((2016, 7, 2), Ans::Number(260));
    answers.insert((2016, 8, 1), Ans::Number(110));
    answers.insert((2016, 8, 2), Ans::Str("ZJHRKCPLYJ"));
    answers.insert((2016, 9, 1), Ans::Number(70186));
    answers.insert((2016, 9, 2), Ans::Number(10915059201));
    answers.insert((2016, 10, 1), Ans::Number(181));
    answers.insert((2016, 10, 2), Ans::Number(12567));

    for y in 2015..=2022 {
        for d in 1..=25 {
            for p in 1..=2 {
                let entry = answers.entry((y, d, p));
                if let Entry::Occupied(entry) = entry {
                    let s: &str = &fill_test_template(y, d, p, *entry.get());
                    println!("{}\n", s);
                }
            }
        }
    }
}
