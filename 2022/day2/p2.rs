use std::fs;

fn main() {
    // opponent:
    // A : rock
    // B : paper
    // C : scissors
    // response:
    // X : loose
    // Y : draw
    // Z : win
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
            match player {
                1 => 0 + if op - 1 == 0 { 3 } else { op - 1 },
                2 => 3 + op,
                _ => 6 + if op + 1 == 4 { 1 } else { op + 1 },
            }
        })
        .into_iter()
        .sum::<u32>();
    println!("{:?}", input);
}
