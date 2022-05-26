use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
struct Point(i32, i32);

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[allow(dead_code)]
pub fn main(input: &str) {

    let input: Vec<Dir> = input.chars().map(|x| {
        match x {
            '^' => Dir::Up,
            '>' => Dir::Right,
            'v' => Dir::Down,
            '<' => Dir::Left,
             _  => panic!("Invalid char.")
        }
    }).collect();    

    let mut hm: HashMap<Point, i32> = HashMap::new();
    let mut loc: Point = Point(0, 0);
    let mut loc2: Point = Point(0, 0);
    hm.entry(loc).or_insert(1);

    let mut toggle: bool = true;

    for d in input {
        if toggle {
            loc = match d {
                Dir::Up    => Point(loc.0, loc.1 - 1), 
                Dir::Right => Point(loc.0 + 1, loc.1), 
                Dir::Down  => Point(loc.0, loc.1 + 1),
                Dir::Left  => Point(loc.0 - 1, loc.1),
            };
            let count = hm.entry(loc).or_insert(0);
            *count += 1;
        } else {
            loc2 = match d {
                Dir::Up    => Point(loc2.0, loc2.1 - 1), 
                Dir::Right => Point(loc2.0 + 1, loc2.1), 
                Dir::Down  => Point(loc2.0, loc2.1 + 1),
                Dir::Left  => Point(loc2.0 - 1, loc2.1),
            };
            let count = hm.entry(loc2).or_insert(0);
            *count += 1;
        }

        toggle = !toggle;
    }

    println!("{}", hm.len());

}
