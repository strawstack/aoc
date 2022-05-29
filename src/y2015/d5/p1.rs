use std::collections::HashSet;

fn check_word(word: &str) -> bool {
    let mut vowel_count: i32 = 0;
    let mut twice_in_a_row: bool = false;
    let ban_list: HashSet<&str> = HashSet::from(["ab", "cd", "pq", "xy"]);

    for i in 0..word.chars().count() {
        if i < word.chars().count() - 1 {
            let ban_word_maybe: &str = &word[i..=i + 1];
            if ban_list.contains(ban_word_maybe) {
                return false;
            }

            if word.chars().nth(i) == word.chars().nth(i + 1) {
                twice_in_a_row = true;
            }
        }
        if "aeiou".contains(word.chars().nth(i).unwrap()) {
            vowel_count += 1;
        }
    }
    vowel_count >= 3 && twice_in_a_row
}

#[allow(dead_code)]
pub fn main(input: &str) -> i32 {
    let input = input.lines();
    let mut count: i32 = 0;
    for word in input {
        if check_word(word) {
            count += 1;
        }
    }

    println!("{}", count);
    count
}
