use bigint::U512;

pub fn parse(s: &str) -> Box<dyn Fn(&U512) -> bool> {
    assert!(s.starts_with("divisible by "));
    let operand = &s[13..];
    let operand = U512::from(operand.parse::<u32>().unwrap());
    Box::new(move |input| {
        let (res, overflow) = input.overflowing_rem(operand);
        assert!(!overflow);
        res == U512::from(0)
    })
}
