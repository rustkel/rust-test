mod counter;
mod poker_hand;
mod simple_print;
mod splitter;

fn main() {
    let hands = vec!["🂡 🂮 🂭 🂫 🂪", "🃂 🃞 🃍 🃁 🃊"];
    for hand in hands {
        p!(hand, poker_hand::poker_hand(hand));
    }

    let g: Vec<char> = "A small test".chars().collect();
    let res = counter::counter(&g);
    println!("{:?}", res);
    println!("{}", res[&'t']);
}
