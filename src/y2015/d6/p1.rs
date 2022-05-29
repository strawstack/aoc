use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
enum Cmd {
    Toggle,
    On,
    Off,
}

#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Op {
    cmd: Cmd,
    from: Point,
    to: Point,
}

fn parse_line(re: &Regex, line: &str) -> Op {
    let items: Vec<&str> = re.split(line).collect();
    match items[0] {
        "toggle" => Op {
            cmd: Cmd::Toggle,
            from: Point(items[1].parse().unwrap(), items[2].parse().unwrap()),
            to: Point(items[4].parse().unwrap(), items[5].parse().unwrap()),
        },
        "turn" => match items[1] {
            "on" => Op {
                cmd: Cmd::On,
                from: Point(items[2].parse().unwrap(), items[3].parse().unwrap()),
                to: Point(items[5].parse().unwrap(), items[6].parse().unwrap()),
            },
            "off" => Op {
                cmd: Cmd::Off,
                from: Point(items[2].parse().unwrap(), items[3].parse().unwrap()),
                to: Point(items[5].parse().unwrap(), items[6].parse().unwrap()),
            },
            _ => panic!("Invalid command type."),
        },
        _ => panic!("Invalid command name."),
    }
}

#[allow(dead_code)]
pub fn main(input: &str) -> i32 {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    let re = Regex::new(r" |,").unwrap();
    for line in input.lines() {
        let op: Op = parse_line(&re, line);
        for r in op.from.0..=op.to.0 {
            for c in op.from.1..=op.to.1 {
                match op.cmd {
                    Cmd::Toggle => {
                        if grid.contains(&(r, c)) {
                            grid.remove(&(r, c));
                        } else {
                            grid.insert((r, c));
                        }
                    }
                    Cmd::On => {
                        grid.insert((r, c));
                    }
                    Cmd::Off => {
                        grid.remove(&(r, c));
                    }
                }
            }
        }
    }

    println!("{}", grid.len());
    grid.len().try_into().unwrap()
}
