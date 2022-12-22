use std::fs;

fn main() {
    let mut input = fs::read_to_string("input.txt")
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
        .collect::<Vec<_>>();
    input.sort_by(|a, b| b.cmp(a));
    //let len = inputvec.len();
    println!("{}", input[0] + input[1] + input[2]);
}
