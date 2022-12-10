mod device;

use device::{Device, Op};

fn main() {
    let stdin = util::Stdin::new();
    let pipeline = stdin.cleaned_lines().map(|line| Op::from(line.as_str()));
    let mut device = Device::new(pipeline);
    let sig_strengths: isize = [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|clock| {
            tick_until_clock(&mut device, *clock);
            (device.clock as isize) * device.r0
        })
        .sum();
    println!(
        "cumulative signal strength is {} at clock {}",
        sig_strengths, device.clock
    );

    tick_until_clock(&mut device, 240);
    device.draw_crt();
}

fn tick_until_clock<T: Iterator<Item = Op>>(device: &mut Device<T>, clock: usize) {
    for _ in device.clock..clock {
        device.tick();
    }
}
