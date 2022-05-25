use std::cmp::min;

#[allow(dead_code)]
pub fn main(input: &str) {
    let lst: Vec<Vec<i32>> = input
        .lines()
        .map(|x| x.split('x').map(|y| y.parse().unwrap()).collect())
        .collect();

    let mut total: i32 = 0;

    for rect in lst {
        let l: i32 = rect[0];
        let w: i32 = rect[1];
        let h: i32 = rect[2];
        let s1: i32 = l * w;
        let s2: i32 = w * h;
        let s3: i32 = h * l;
        let side_area: i32 = 2 * s1 + 2 * s2 + 2 * s3;
        let smallest = min(s1, min(s2, s3));
        total += side_area + smallest;
    }

    println!("{}", total);
}
