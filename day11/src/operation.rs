use rug::Integer;
use std::ops::{AddAssign, MulAssign};

pub fn parse(s: &str) -> Box<dyn Fn(&mut Integer)> {
    assert!(s.starts_with("new = old "));
    let mutation = &s[10..];
    let (operator, operand) = mutation.split_once(' ').unwrap();
    let operator: fn(&mut Integer, Integer) = match operator {
        "+" => AddAssign::add_assign,
        "*" => MulAssign::mul_assign,
        other => panic!("unknown operator {}", other),
    };
    let operand = if operand == "old" {
        None
    } else {
        Some(operand.parse::<Integer>().unwrap())
    };
    Box::new(move |item: &mut Integer| {
        if let Some(operand) = operand.as_ref() {
            operator(item, operand.clone());
        } else {
            operator(item, item.clone());
        }
    })
}
