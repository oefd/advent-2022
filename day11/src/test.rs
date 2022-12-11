pub fn parse(s: &str) -> Box<dyn Fn(usize) -> bool> {
    assert!(s.starts_with("divisible by "));
    let operand = &s[13..];
    let operand = operand.parse::<usize>().unwrap();
    Box::new(move |input| input % operand == 0)
}
