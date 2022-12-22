use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|s| {
            s.trim()
                .lines()
                .flat_map(str::parse::<i32>)
                .collect::<Vec<_>>()
                .into_iter()
                .sum::<i32>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .max()
        .unwrap();
    println!("{:?}", input);
}
