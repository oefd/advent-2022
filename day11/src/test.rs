use rug::Integer;

pub fn parse(s: &str) -> Box<dyn Fn(&Integer) -> bool> {
    assert!(s.starts_with("divisible by "));
    let operand = &s[13..];
    let operand = operand.parse::<u32>().unwrap();
    Box::new(move |input| input.mod_u(operand) == 0)
}
