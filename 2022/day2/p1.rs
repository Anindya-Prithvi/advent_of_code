use std::fs;

fn main() {
    // opponent:
    // A : rock
    // B : paper
    // C : scissors
    // response:
    // X : rock
    // Y : paper
    // Z : scissors
    // score:
    // rock : 1
    // paper : 2
    // scissors : 3
    // win : 6
    // lose : 0
    // draw : 3
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|x| {
            let op: u32 = x.bytes().nth(0).unwrap() as u32 - 64;
            let player: u32 = x.bytes().nth(2).unwrap() as u32 - 87;
            player
                + if op == player {
                    3
                } else if op % 3 == player - 1 {
                    6
                } else {
                    0
                }
        })
        .into_iter()
        .sum::<u32>();
    println!("{:?}", input);
}
