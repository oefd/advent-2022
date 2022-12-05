#![feature(iter_array_chunks)]
mod deck;
mod movement;

use deck::DeckBuilder;
use movement::Movement;

fn main() {
    let stdin = util::Stdin::new();
    let mut input = stdin.cleaned_lines();

    let mut deck = {
        let mut deck_builder = DeckBuilder::new();
        input
            .by_ref()
            .take_while(|line| !line.starts_with(" 1 "))
            .for_each(|line| deck_builder.feed_line(line.as_str()));
        deck_builder.build()
    };

    input
        .map(|line| Movement::from(line.as_str()))
        .for_each(|movement| deck.make_movement_9000(&movement));

    println!("top crates are {}", deck.top_crates());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_move() {
        let input = "[A] [B] [C]";
        let mut deck = {
            let mut builder = DeckBuilder::new();
            builder.feed_line(input);
            builder.build()
        };
        deck.make_movement_9000(&Movement {
            count: 1,
            source: 1,
            destination: 3,
        });
        assert!(deck.top_crates() == " BA");
    }

    #[test]
    fn multilevel_move_9000() {
        let mut deck = {
            let mut builder = DeckBuilder::new();
            for line in ["    [E] [F]    ", "[A] [B] [C] [D]"] {
                builder.feed_line(line);
            }
            builder.build()
        };
        assert!(deck.top_crates() == "AEFD");
        deck.make_movement_9000(&Movement {
            count: 2,
            source: 2,
            destination: 3,
        });
        assert!(deck.top_crates() == "A BD");
    }

    #[test]
    fn multilevel_move_9001() {
        let mut deck = {
            let mut builder = DeckBuilder::new();
            for line in ["    [E] [F]    ", "[A] [B] [C] [D]"] {
                builder.feed_line(line);
            }
            builder.build()
        };
        assert!(deck.top_crates() == "AEFD");
        deck.make_movement_9001(&Movement {
            count: 2,
            source: 2,
            destination: 3,
        });
        assert!(deck.top_crates() == "A ED");
    }
}
