use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input")
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (dimx, dimy) = (input.len(), input[0].len());
    let mut sumoflowpoints: u32 = 0;
    for i in 0..dimx {
        for j in 0..dimy {
            let mut ismin = true;
            if i > 0 {
                ismin = ismin && input[i][j] < input[i - 1][j];
            }
            if i < dimx - 1 {
                ismin = ismin && input[i][j] < input[i + 1][j];
            }
            if j > 0 {
                ismin = ismin && input[i][j] < input[i][j - 1];
            }
            if j < dimy - 1 {
                ismin = ismin && input[i][j] < input[i][j + 1];
            }
            if ismin {
                sumoflowpoints += input[i][j] + 1;
            }
        }
    }
    println!("{}", sumoflowpoints);
}
