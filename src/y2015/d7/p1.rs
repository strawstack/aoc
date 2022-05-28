use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

const D: bool = false; 

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Operator {
    Not,
    Or,
    And,
    RShift,
    LShift,
}

type Wire = str;
type Cache<'a> = HashMap<&'a Wire, u16>;
type RMCache<'a> = Rc<Mutex<HashMap<&'a Wire, u16>>>;

#[derive(Debug)]
#[derive(Copy, Clone)]
enum Value<'a> {
    Number(u16),
    Wire(&'a Wire),
}

#[derive(Debug)]
enum Gate<'a> {
    Input {
        value: Value<'a>,
    },
    Unary {
        op: Operator,
        value: Value<'a>,
    },
    Binary {
        op: Operator,
        left_value: Value<'a>,
        right_value: Value<'a>,
    },
}

fn apply_operator(op: Operator, value: u16) -> u16 {
    if op != Operator::Not { 
        panic!("Apply operator fails.") 
    }
    if D { 
        println!("value: {}, !value: {}", value, !value);
    }
    !value
}

fn apply_binary_operator(op: Operator, left_value: u16, right_value: u16) -> u16 {
    match op {
        Operator::Not => panic!("Apply binary operator fails."),
        Operator::Or => {
            if D { println!("{} | {}", left_value, right_value); }
            left_value | right_value
        },
        Operator::And => {
            if D { println!("{} & {}", left_value, right_value); }
            left_value & right_value
        },
        Operator::RShift => {
            if D { println!("{} >> {}", left_value, right_value); }
            left_value >> right_value
        },
        Operator::LShift => {
            if D { println!("{} << {}", left_value, right_value); }
            left_value << right_value
        },
    }
}

fn unpack<'a>(hm: &'a HashMap<&Wire, Gate>, ch: RMCache<'a>, value: Value<'a>) -> u16 {
    if let Value::Number(number) = value {
        return number;

    } else if let Value::Wire(wire) = value {
        return compute_wire(&hm, ch, wire);

    } else {
        panic!("Input gate if let value fails.")

    }
}

fn compute_wire<'a>(hm: &'a HashMap<&Wire, Gate>, ch: RMCache<'a>, wire: &'a Wire) -> u16 {
    
    if D {
        println!("Wire: {}, Gate: {:?}", wire, hm.get(wire).unwrap());
    }

    {
        let cm = ch.lock().unwrap();
        if let Some(value) = cm.get(wire) {
            return *value;
        }
    }

    let gate = hm.get(wire).unwrap();   
    if let Gate::Input { value } = gate {
        let res = unpack(&hm, Rc::clone(&ch), *value);
        {
            let mut cm = ch.lock().unwrap();
            cm.insert(wire, res);
        }
        return res; 

    } else if let Gate::Unary { op, value } = gate {
        let res = apply_operator(*op, unpack(&hm, Rc::clone(&ch), *value));
        {
            let mut cm = ch.lock().unwrap();
            cm.insert(wire, res);
        }
        return res; 
        
    } else if let Gate::Binary { op, left_value, right_value } = gate {
        let res = apply_binary_operator(*op, unpack(&hm, Rc::clone(&ch), *left_value), unpack(&hm, Rc::clone(&ch), *right_value));
        {
            let mut cm = ch.lock().unwrap();
            cm.insert(wire, res);
        }
        return res;
        
    } else {
        panic!("Gate match fails.")
    }
}

fn make_gate<'a>(line: &'a str) -> Gate<'a> {
    let line: Vec<&str> = line.split(" ").collect();
    match line.len() {
        1 => {
            let res = line[0].parse::<u16>();
            Gate::Input {
                value: match res {
                    Ok(v) => Value::Number(v),
                    Err(_) => Value::Wire(line[0]),
                }
            }
        },
        2 => {
            Gate::Unary {
                op: Operator::Not,
                value: match line[1].parse::<u16>() {
                    Ok(v) => Value::Number(v),
                    Err(_) => Value::Wire(line[1]), 
                },
            }
        },
        3 => {
            Gate::Binary {
                op: match line[1] {
                    "OR" => Operator::Or, 
                    "AND" => Operator::And,
                    "LSHIFT" => Operator::LShift,
                    "RSHIFT" => Operator::RShift,
                    _ => panic!("Binary operator match fails.")
                },
                left_value: match line[0].parse::<u16>() {
                    Ok(v) => Value::Number(v),
                    Err(_) => Value::Wire(line[0]),
                },
                right_value: match line[2].parse::<u16>() {
                    Ok(v) => Value::Number(v),
                    Err(_) => Value::Wire(line[2]),
                },
            }
        },
        _ => panic!("Match line length fails")
    }
}

#[allow(dead_code)]
pub fn main(input: &str) -> i32 {
    
    let mut hm: HashMap<&Wire, Gate> = HashMap::new(); 
   
    for line in input.lines() {
        let split: Vec<&str> = line.split(" -> ").collect();
        hm.insert(split[1], make_gate(split[0]));
    }

    let ch: Rc<Mutex<Cache>> = Rc::new(Mutex::new(HashMap::new())); 

    let ans: i32 = compute_wire(&hm, ch, "a").into();
    println!("{:?}", ans);
    ans
}
