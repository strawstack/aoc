use std::env;
mod data;
mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2020;
mod y2021;
mod y2022;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (year, day, part): (i32, i32, i32) = (
        args[1].parse().unwrap(),
        args[2].parse().unwrap(),
        args[3].parse().unwrap(),
    );

    let s: &str = &data::get(year, day);

    match year {
        2015 => {
            use y2015::*;
            match day {
                1 => {
                    use d1::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                2 => {
                    use d2::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                3 => {
                    use d3::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                4 => {
                    use d4::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                5 => {
                    use d5::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                6 => {
                    use d6::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                7 => {
                    use d7::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                8 => {
                    use d8::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                9 => {
                    use d9::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                10 => {
                    use d10::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                11 => {
                    use d11::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                12 => {
                    use d12::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                13 => {
                    use d13::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                14 => {
                    use d14::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                15 => {
                    use d15::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                16 => {
                    use d16::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                17 => {
                    use d17::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                18 => {
                    use d18::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                19 => {
                    use d19::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                20 => {
                    use d20::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                21 => {
                    use d21::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                22 => {
                    use d22::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                23 => {
                    use d23::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                24 => {
                    use d24::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                25 => {
                    use d25::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid day.");
                }
            }
        }
        2016 => {
            use y2016::*;
            match day {
                1 => {
                    use d1::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                2 => {
                    use d2::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                3 => {
                    use d3::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                4 => {
                    use d4::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                5 => {
                    use d5::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                6 => {
                    use d6::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                7 => {
                    use d7::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                8 => {
                    use d8::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                9 => {
                    use d9::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                10 => {
                    use d10::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                11 => {
                    use d11::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                12 => {
                    use d12::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                13 => {
                    use d13::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                14 => {
                    use d14::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                15 => {
                    use d15::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                16 => {
                    use d16::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                17 => {
                    use d17::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                18 => {
                    use d18::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                19 => {
                    use d19::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                20 => {
                    use d20::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                21 => {
                    use d21::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                22 => {
                    use d22::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                23 => {
                    use d23::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                24 => {
                    use d24::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                25 => {
                    use d25::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid day.");
                }
            }
        }
        2017 => {
            use y2017::*;
            match day {
                1 => {
                    use d1::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                2 => {
                    use d2::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                3 => {
                    use d3::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                4 => {
                    use d4::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                5 => {
                    use d5::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                6 => {
                    use d6::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                7 => {
                    use d7::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                8 => {
                    use d8::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                9 => {
                    use d9::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                10 => {
                    use d10::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                11 => {
                    use d11::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                12 => {
                    use d12::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                13 => {
                    use d13::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                14 => {
                    use d14::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                15 => {
                    use d15::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                16 => {
                    use d16::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                17 => {
                    use d17::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                18 => {
                    use d18::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                19 => {
                    use d19::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                20 => {
                    use d20::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                21 => {
                    use d21::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                22 => {
                    use d22::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                23 => {
                    use d23::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                24 => {
                    use d24::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                25 => {
                    use d25::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid day.");
                }
            }
        }
        2018 => {
            use y2018::*;
            match day {
                1 => {
                    use d1::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                2 => {
                    use d2::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                3 => {
                    use d3::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                4 => {
                    use d4::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                5 => {
                    use d5::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                6 => {
                    use d6::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                7 => {
                    use d7::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                8 => {
                    use d8::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                9 => {
                    use d9::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                10 => {
                    use d10::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                11 => {
                    use d11::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                12 => {
                    use d12::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                13 => {
                    use d13::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                14 => {
                    use d14::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                15 => {
                    use d15::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                16 => {
                    use d16::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                17 => {
                    use d17::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                18 => {
                    use d18::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                19 => {
                    use d19::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                20 => {
                    use d20::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                21 => {
                    use d21::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                22 => {
                    use d22::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                23 => {
                    use d23::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                24 => {
                    use d24::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                25 => {
                    use d25::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid day.");
                }
            }
        }
        2019 => {
            use y2019::*;
            match day {
                1 => {
                    use d1::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                2 => {
                    use d2::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                3 => {
                    use d3::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                4 => {
                    use d4::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                5 => {
                    use d5::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                6 => {
                    use d6::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                7 => {
                    use d7::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                8 => {
                    use d8::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                9 => {
                    use d9::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                10 => {
                    use d10::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                11 => {
                    use d11::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                12 => {
                    use d12::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                13 => {
                    use d13::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                14 => {
                    use d14::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                15 => {
                    use d15::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                16 => {
                    use d16::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                17 => {
                    use d17::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                18 => {
                    use d18::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                19 => {
                    use d19::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                20 => {
                    use d20::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                21 => {
                    use d21::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                22 => {
                    use d22::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                23 => {
                    use d23::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                24 => {
                    use d24::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                25 => {
                    use d25::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid day.");
                }
            }
        }
        2020 => {
            use y2020::*;
            match day {
                1 => {
                    use d1::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                2 => {
                    use d2::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                3 => {
                    use d3::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                4 => {
                    use d4::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                5 => {
                    use d5::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                6 => {
                    use d6::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                7 => {
                    use d7::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                8 => {
                    use d8::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                9 => {
                    use d9::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                10 => {
                    use d10::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                11 => {
                    use d11::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                12 => {
                    use d12::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                13 => {
                    use d13::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                14 => {
                    use d14::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                15 => {
                    use d15::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                16 => {
                    use d16::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                17 => {
                    use d17::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                18 => {
                    use d18::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                19 => {
                    use d19::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                20 => {
                    use d20::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                21 => {
                    use d21::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                22 => {
                    use d22::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                23 => {
                    use d23::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                24 => {
                    use d24::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                25 => {
                    use d25::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid day.");
                }
            }
        }
        2021 => {
            use y2021::*;
            match day {
                1 => {
                    use d1::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                2 => {
                    use d2::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                3 => {
                    use d3::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                4 => {
                    use d4::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                5 => {
                    use d5::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                6 => {
                    use d6::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                7 => {
                    use d7::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                8 => {
                    use d8::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                9 => {
                    use d9::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                10 => {
                    use d10::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                11 => {
                    use d11::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                12 => {
                    use d12::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                13 => {
                    use d13::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                14 => {
                    use d14::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                15 => {
                    use d15::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                16 => {
                    use d16::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                17 => {
                    use d17::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                18 => {
                    use d18::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                19 => {
                    use d19::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                20 => {
                    use d20::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                21 => {
                    use d21::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                22 => {
                    use d22::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                23 => {
                    use d23::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                24 => {
                    use d24::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                25 => {
                    use d25::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid day.");
                }
            }
        }
        2022 => {
            use y2022::*;
            match day {
                1 => {
                    use d1::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                2 => {
                    use d2::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                3 => {
                    use d3::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                4 => {
                    use d4::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                5 => {
                    use d5::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                6 => {
                    use d6::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                7 => {
                    use d7::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                8 => {
                    use d8::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                9 => {
                    use d9::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                10 => {
                    use d10::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                11 => {
                    use d11::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                12 => {
                    use d12::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                13 => {
                    use d13::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                14 => {
                    use d14::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                15 => {
                    use d15::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                16 => {
                    use d16::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                17 => {
                    use d17::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                18 => {
                    use d18::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                19 => {
                    use d19::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                20 => {
                    use d20::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                21 => {
                    use d21::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                22 => {
                    use d22::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                23 => {
                    use d23::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                24 => {
                    use d24::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                25 => {
                    use d25::*;
                    match part {
                        1 => {
                            p1::main(s);
                        }
                        2 => {
                            p2::main(s);
                        }
                        _ => {
                            panic!("Invalid part.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid day.");
                }
            }
        }
        _ => {
            panic!("Invalid year.");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn y2015_d1_p1() {
        let s: &str = &data::get(2015, 1);
        assert_eq!(y2015::d1::p1::main(s), 232);
    }

    #[test]
    fn y2015_d1_p2() {
        let s: &str = &data::get(2015, 1);
        assert_eq!(y2015::d1::p2::main(s), 1783);
    }

    #[test]
    fn y2015_d2_p1() {
        let s: &str = &data::get(2015, 2);
        assert_eq!(y2015::d2::p1::main(s), 1606483);
    }

    #[test]
    fn y2015_d2_p2() {
        let s: &str = &data::get(2015, 2);
        assert_eq!(y2015::d2::p2::main(s), 3842356);
    }

    #[test]
    fn y2015_d3_p1() {
        let s: &str = &data::get(2015, 3);
        assert_eq!(y2015::d3::p1::main(s), 2081);
    }

    #[test]
    fn y2015_d3_p2() {
        let s: &str = &data::get(2015, 3);
        assert_eq!(y2015::d3::p2::main(s), 2341);
    }

    #[test]
    fn y2015_d4_p1() {
        let s: &str = &data::get(2015, 4);
        assert_eq!(y2015::d4::p1::main(s), 254575);
    }

    #[test]
    fn y2015_d4_p2() {
        let s: &str = &data::get(2015, 4);
        assert_eq!(y2015::d4::p2::main(s), 1038736);
    }

    #[test]
    fn y2015_d5_p1() {
        let s: &str = &data::get(2015, 5);
        assert_eq!(y2015::d5::p1::main(s), 238);
    }

    #[test]
    fn y2015_d5_p2() {
        let s: &str = &data::get(2015, 5);
        assert_eq!(y2015::d5::p2::main(s), 69);
    }

    #[test]
    fn y2015_d6_p1() {
        let s: &str = &data::get(2015, 6);
        assert_eq!(y2015::d6::p1::main(s), 377891);
    }

    #[test]
    fn y2015_d6_p2() {
        let s: &str = &data::get(2015, 6);
        assert_eq!(y2015::d6::p2::main(s), 14110788);
    }

    #[test]
    fn y2015_d7_p1() {
        let s: &str = &data::get(2015, 7);
        assert_eq!(y2015::d7::p1::main(s), 3176);
    }

    #[test]
    fn y2015_d7_p2() {
        let s: &str = &data::get(2015, 7);
        assert_eq!(y2015::d7::p2::main(s), 14710);
    }

    #[test]
    fn y2015_d8_p1() {
        let s: &str = &data::get(2015, 8);
        assert_eq!(y2015::d8::p1::main(s), 1371);
    }

    #[test]
    fn y2015_d8_p2() {
        let s: &str = &data::get(2015, 8);
        assert_eq!(y2015::d8::p2::main(s), 2117);
    }

    #[test]
    fn y2015_d9_p1() {
        let s: &str = &data::get(2015, 9);
        assert_eq!(y2015::d9::p1::main(s), 117);
    }

    #[test]
    fn y2015_d9_p2() {
        let s: &str = &data::get(2015, 9);
        assert_eq!(y2015::d9::p2::main(s), 909);
    }

    #[test]
    fn y2015_d10_p1() {
        let s: &str = &data::get(2015, 10);
        assert_eq!(y2015::d10::p1::main(s), 492982);
    }

    #[test]
    fn y2015_d10_p2() {
        let s: &str = &data::get(2015, 10);
        assert_eq!(y2015::d10::p2::main(s), 6989950);
    }

    #[test]
    fn y2015_d11_p1() {
        let s: &str = &data::get(2015, 11);
        assert_eq!(y2015::d11::p1::main(s), "vzbxxyzz");
    }

    #[test]
    fn y2015_d11_p2() {
        let s: &str = &data::get(2015, 11);
        assert_eq!(y2015::d11::p2::main(s), "vzcaabcc");
    }

    #[test]
    fn y2015_d12_p1() {
        let s: &str = &data::get(2015, 12);
        assert_eq!(y2015::d12::p1::main(s), 191164);
    }

    #[test]
    fn y2015_d12_p2() {
        let s: &str = &data::get(2015, 12);
        assert_eq!(y2015::d12::p2::main(s), 87842);
    }

    #[test]
    fn y2015_d13_p1() {
        let s: &str = &data::get(2015, 13);
        assert_eq!(y2015::d13::p1::main(s), 618);
    }

    #[test]
    fn y2015_d13_p2() {
        let s: &str = &data::get(2015, 13);
        assert_eq!(y2015::d13::p2::main(s), 601);
    }

    #[test]
    fn y2015_d14_p1() {
        let s: &str = &data::get(2015, 14);
        assert_eq!(y2015::d14::p1::main(s), 2640);
    }

    #[test]
    fn y2015_d14_p2() {
        let s: &str = &data::get(2015, 14);
        assert_eq!(y2015::d14::p2::main(s), 1102);
    }

    #[test]
    fn y2015_d15_p1() {
        let s: &str = &data::get(2015, 15);
        assert_eq!(y2015::d15::p1::main(s), 21367368);
    }

    #[test]
    fn y2015_d15_p2() {
        let s: &str = &data::get(2015, 15);
        assert_eq!(y2015::d15::p2::main(s), 1766400);
    }

    #[test]
    fn y2016_d1_p1() {
        let s: &str = &data::get(2016, 1);
        assert_eq!(y2016::d1::p1::main(s), 234);
    }

    #[test]
    fn y2016_d1_p2() {
        let s: &str = &data::get(2016, 1);
        assert_eq!(y2016::d1::p2::main(s), 113);
    }

    #[test]
    fn y2016_d2_p1() {
        let s: &str = &data::get(2016, 2);
        assert_eq!(y2016::d2::p1::main(s), 33444);
    }

    #[test]
    fn y2016_d2_p2() {
        let s: &str = &data::get(2016, 2);
        assert_eq!(y2016::d2::p2::main(s), "446A6");
    }

    #[test]
    fn y2016_d3_p1() {
        let s: &str = &data::get(2016, 3);
        assert_eq!(y2016::d3::p1::main(s), 982);
    }

    #[test]
    fn y2016_d3_p2() {
        let s: &str = &data::get(2016, 3);
        assert_eq!(y2016::d3::p2::main(s), 1826);
    }

    #[test]
    fn y2016_d4_p1() {
        let s: &str = &data::get(2016, 4);
        assert_eq!(y2016::d4::p1::main(s), 173787);
    }

    #[test]
    fn y2016_d4_p2() {
        let s: &str = &data::get(2016, 4);
        assert_eq!(y2016::d4::p2::main(s), 548);
    }

    #[test]
    fn y2016_d5_p1() {
        let s: &str = &data::get(2016, 5);
        assert_eq!(y2016::d5::p1::main(s), "f97c354d");
    }

    #[test]
    fn y2016_d5_p2() {
        let s: &str = &data::get(2016, 5);
        assert_eq!(y2016::d5::p2::main(s), "863dde27");
    }

    #[test]
    fn y2016_d6_p1() {
        let s: &str = &data::get(2016, 6);
        assert_eq!(y2016::d6::p1::main(s), "usccerug");
    }

    #[test]
    fn y2016_d6_p2() {
        let s: &str = &data::get(2016, 6);
        assert_eq!(y2016::d6::p2::main(s), "cnvvtafc");
    }

    #[test]
    fn y2016_d7_p1() {
        let s: &str = &data::get(2016, 7);
        assert_eq!(y2016::d7::p1::main(s), 118);
    }

    #[test]
    fn y2016_d7_p2() {
        let s: &str = &data::get(2016, 7);
        assert_eq!(y2016::d7::p2::main(s), 260);
    }

    #[test]
    fn y2016_d8_p1() {
        let s: &str = &data::get(2016, 8);
        assert_eq!(y2016::d8::p1::main(s), 110);
    }

    #[test]
    fn y2016_d8_p2() {
        let s: &str = &data::get(2016, 8);
        assert_eq!(y2016::d8::p2::main(s), "ZJHRKCPLYJ");
    }

    #[test]
    fn y2016_d9_p1() {
        let s: &str = &data::get(2016, 9);
        assert_eq!(y2016::d9::p1::main(s), 70186);
    }

    #[test]
    fn y2016_d9_p2() {
        let s: &str = &data::get(2016, 9);
        assert_eq!(y2016::d9::p2::main(s), 10915059201);
    }

    #[test]
    fn y2016_d10_p1() {
        let s: &str = &data::get(2016, 10);
        assert_eq!(y2016::d10::p1::main(s), 181);
    }

    #[test]
    fn y2016_d10_p2() {
        let s: &str = &data::get(2016, 10);
        assert_eq!(y2016::d10::p2::main(s), 12567);
    }
}
