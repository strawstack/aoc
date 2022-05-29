use regex::Regex;

#[derive(Debug)]
enum Direction { R, L }

#[allow(dead_code)]
#[derive(Debug)]
struct Move {
    dir: Direction,
    steps: i32,
}

#[allow(dead_code)]
pub fn main(input: &str) -> i32 {
    let re = Regex::new(r"^(R|L)(\d+)$").unwrap();
    let input: Vec<Move> = input.trim().split(", ").map(|x| {
        let caps = re.captures(x).unwrap();
        let letter = caps.get(1).map_or("", |x| x.as_str());
        let steps: i32 = caps.get(2).map_or(0, |x| x.as_str().parse().unwrap());
        Move {
            dir: if letter == "R" { Direction::R } else { Direction::L },
            steps: steps, 
        }
    }).collect();

    let mut facing: i32 = 0;
    let mut pos: (i32, i32) = (0, 0);     

    for mv in input {
        match mv.dir {
            Direction::R => facing += 1,
            Direction::L => facing -= 1,
        }
        if facing == 4 {
            facing = 0;
        }
        if facing == -1 {
            facing = 3;
        }

        match facing {
            0 => {
                pos = (pos.0, pos.1 - mv.steps);                
            },
            1 => {
                pos = (pos.0 + mv.steps, pos.1);                
            },
            2 => {
                pos = (pos.0, pos.1 + mv.steps);                
            },
            3 => {
                pos = (pos.0 - mv.steps, pos.1);                
            },
            _ => panic!("Match facing fails"),
        }
    }
    let ans = pos.0.abs() + pos.1.abs();
    println!("{:?}", ans);
    ans
}
