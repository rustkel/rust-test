mod poker_hand;

fn main() {
    let hands = vec![
        "🂡 🂮 🂭 🂫 🂪",
        "🃏 🃂 🂢 🂮 🃍",
        "🃏 🂵 🃇 🂨 🃉",
        "🃏 🃂 🂣 🂤 🂥",
        "🃏 🂳 🃂 🂣 🃃",
        "🃏 🂷 🃂 🂣 🃃",
        "🃏 🂷 🃇 🂧 🃗",
        "🃏 🂻 🂽 🂾 🂱",
        "🃏 🃔 🃞 🃅 🂪",
        "🃏 🃞 🃗 🃖 🃔",
        "🃏 🃂 🃟 🂤 🂥",
        "🃏 🃍 🃟 🂡 🂪",
        "🃏 🃍 🃟 🃁 🃊",
        "🃏 🃂 🂢 🃟 🃍",
        "🃏 🃂 🂢 🃍 🃍",
        "🃂 🃞 🃍 🃁 🃊",
    ];
    for hand in hands{
        println!("{} {}", hand, poker_hand::poker_hand(hand));
    }
}

