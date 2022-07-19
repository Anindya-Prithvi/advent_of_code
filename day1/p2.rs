use std::fs;

fn main() {
    let inproc = fs::read_to_string("input1.txt")
        .expect("Failed to read file")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    // now we have the whole array
    let length = inproc.len();
    let mut totsum = 0;
    let mut threesome = Vec::<i32>::new();

    for i in 0..length - 2 {
        threesome.push(inproc[i] + inproc[i + 1] + inproc[i + 2]);
    }

    for i in 1..length - 2 {
        if threesome[i] > threesome[i - 1] {
            totsum += 1;
        }
    }

    println!("{}", totsum);
}
