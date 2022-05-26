use std::collections::HashMap;

fn check_word(word: &str) -> bool {

    let mut xyx = false;
    let mut pair = false;
    let len: i32 = word.chars().count() as i32;

    let mut hm: HashMap<&str, i32> = HashMap::new();

    for i in 0..len {
        if i < len - 2 {
            if word.chars().nth(i as usize).unwrap() == word.chars().nth((i + 2) as usize).unwrap() {
                xyx = true;
            }
        }
        if i < len - 1 {
            let slice: &str = &word[(i as usize)..=(i + 1) as usize];
            if hm.contains_key(slice) && hm[slice] + 1 != i {
                pair = true;
            }
            hm.entry(slice).or_insert(i);
        }
    }
    xyx && pair
} 

#[allow(dead_code)]
pub fn main(input: &str) {
    let input = input.lines();
    let mut count: i32 = 0;
    for word in input {
        if check_word(word) {
            count += 1;
        }
    }

    println!("{}", count);
}
