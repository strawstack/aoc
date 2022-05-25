#[allow(dead_code)]
pub fn main(input: &str) {
    let ans: i32 = input.chars().fold(0, |acc, x| if x == '(' { acc + 1 } else { acc - 1 });
    println!("{}", ans);
}
