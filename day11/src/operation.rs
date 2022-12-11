use std::ops::{Add, Mul, Sub, Div};

pub fn parse(s: &str) -> Box<dyn Fn(usize) -> usize> {
    assert!(s.starts_with("new = old "));
    let mutation = &s[10..];
    let (operator, operand) = mutation.split_once(' ').unwrap();
    let operator: fn(usize, usize) -> usize = match operator {
        "+" => Add::add,
        "*" => Mul::mul,
        "-" => Sub::sub,
        "/" => Div::div,
        other => panic!("unknown operator {}", other),
    };
    let operand = if operand == "old" {
        None
    } else {
        Some(operand.parse::<usize>().unwrap())
    };
    Box::new(move |worry| {
        if let Some(operand) = operand.as_ref() {
            operator(worry, *operand)
        } else {
            operator(worry, worry)
        }
    })
}
