use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Unable to read file");
    let splits = input.trim().lines().flat_map(str::parse::<i32>);
    let inputvec = splits.collect::<Vec<i32>>();
    let len = inputvec.len();
    let mut totcount: i32 = 0;
    for i in 1..len {
        if inputvec[i] > inputvec[i - 1] {
            totcount += 1;
        }
    }
    println!("{}", totcount);
}
