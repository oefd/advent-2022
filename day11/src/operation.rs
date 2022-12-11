use bigint::U512;
use std::ops::{Add, Mul};

pub fn parse(s: &str) -> Box<dyn Fn(&U512) -> U512> {
    assert!(s.starts_with("new = old "));
    let mutation = &s[10..];
    let (operator, operand) = mutation.split_once(' ').unwrap();
    let operator: fn(U512, U512) -> U512 = match operator {
        "+" => Add::add,
        "*" => Mul::mul,
        other => panic!("unknown operator {}", other),
    };
    let operand = if operand == "old" {
        None
    } else {
        Some(U512::from(operand.parse::<u64>().unwrap()))
    };
    Box::new(move |item| {
        if let Some(operand) = operand.as_ref() {
            operator(*item, operand.clone())
        } else {
            operator(*item, item.clone())
        }
    })
}
