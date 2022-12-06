mod ring_buf;

use ring_buf::RingBuf;

fn main() {
    let stdin = util::Stdin::new();
    let input = stdin.cleaned_lines().next().unwrap();

    // part 1
    let mut ring = RingBuf::<u8, 4>::new();
    let signal_start = {
        let (idx, _) = input
            .as_bytes()
            .iter()
            .copied()
            .enumerate()
            .find(|(_idx, byte)| {
                ring.write(*byte);
                ring.is_distinct_items()
            })
            .unwrap();
        idx + 1
    };
    println!("start of packet at {}", signal_start);

    // part 2
    let mut ring = RingBuf::<u8, 14>::new();
    let signal_start = {
        let (idx, _) = input
            .as_bytes()
            .iter()
            .copied()
            .enumerate()
            .find(|(_idx, byte)| {
                ring.write(*byte);
                ring.is_distinct_items()
            })
            .unwrap();
        idx + 1
    };
    println!("start of message at {}", signal_start);
}
