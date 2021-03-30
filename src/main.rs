mod complex;
mod poker_hand;
mod simple_print;

fn main() {
    let hands = vec!["🂡 🂮 🂭 🂫 🂪", "🃂 🃞 🃍 🃁 🃊"];
    for hand in hands {
        p!(hand, poker_hand::poker_hand(hand));
    }
}
