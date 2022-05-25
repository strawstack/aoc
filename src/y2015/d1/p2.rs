fn sol(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor == -1 {
            return i as i32;
        }
    }
    -1
}

#[allow(dead_code)]
pub fn main(input: &str) {
    let ans: i32 = sol(input) + 1;
    println!("{}", ans);
}
