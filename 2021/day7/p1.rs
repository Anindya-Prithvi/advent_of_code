use std::fs;

fn main() {
    let mut input = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    input.sort();
    let totalelems = input.len();
    let median = input[totalelems / 2];
    let ans = input.into_iter().fold(0, |a, b| (b - median).abs() + a);
    println!("{}", ans);
}
