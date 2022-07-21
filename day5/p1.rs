use std::cmp;
use std::fs;

fn main() {
    let mut matrix = vec![vec![0; 1000]; 1000];
    let _input = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .for_each(|l| {
            let start_dest = l
                .split("->")
                .map(|s| {
                    s.trim()
                        .split(',')
                        .map(|e| e.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            if start_dest[0][0] == start_dest[1][0] {
                // horizontal
                for i in cmp::min(start_dest[0][1], start_dest[1][1])
                    ..=cmp::max(start_dest[0][1], start_dest[1][1])
                {
                    matrix[start_dest[0][0] as usize][i as usize] += 1;
                }
            } else if start_dest[0][1] == start_dest[1][1] {
                // vertical
                for i in cmp::min(start_dest[0][0], start_dest[1][0])
                    ..=cmp::max(start_dest[0][0], start_dest[1][0])
                {
                    matrix[i as usize][start_dest[0][1] as usize] += 1;
                }
            } else {
                // diagonal
            }
        });
    let ans = matrix.into_iter().fold(0, |acc, v| {
        acc + v
            .into_iter()
            .fold(0, |acc, v| acc + if v >= 2 { 1 } else { 0 })
    });
    println!("{}", ans);
}
