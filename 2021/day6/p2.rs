use std::fs;

fn main() {
    let mut days = vec![0u64; 9];
    let _input = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .for_each(|x| days[x as usize] += 1);
    for i in 0..256 {
        days[(i + 7) % 9] += days[i % 9];
    }
    println!("{}", days.iter().sum::<u64>());
}
