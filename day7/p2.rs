use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let totalelems = input.len() as i32;
    let mean: i32 = input.iter().sum::<i32>() / totalelems;
    let ans = input.into_iter().fold(0, |a, b| {
        let n = (b - mean).abs();
        (n * (n + 1)) / 2 + a
    });
    println!("{}", ans);
}
