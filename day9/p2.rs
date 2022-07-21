use std::fs;

#[derive(Debug)]
struct Point {
    height: u8,
    visited: bool,
    component: i32,
}

fn main() {
    let mut input = fs::read_to_string("input.txt")
        .expect("Failed to read input")
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Point {
                    height: c.to_digit(10).unwrap() as u8,
                    visited: false,
                    component: i32::max_value(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let dimx = input.len();
    let dimy = input[0].len();
    let mut comps = 0;
    for i in 0..dimx {
        for j in 0..dimy {
            if input[i][j].visited == true {
                continue;
            } else {
                if search(i, j, &mut input, comps) {
                    comps += 1;
                }
            }
        }
    }
    println!("{}", comps);
    let mut bigcomps = vec![0; comps as usize];
    for i in 0..dimx {
        for j in 0..dimy {
            if input[i][j].height != 9 {
                bigcomps[input[i][j].component as usize] += 1;
            }
        }
    }

    bigcomps.sort();
    bigcomps.reverse();
    println!("{:?}", bigcomps);
    println!("{}", bigcomps[0] * bigcomps[1] * bigcomps[2]);
}

fn search(x: usize, y: usize, input: &mut Vec<Vec<Point>>, comp: i32) -> bool {
    if input[x][y].visited == true {
        return false;
    }
    if input[x][y].height == 9 {
        return false;
    }
    input[x][y].visited = true;
    input[x][y].component = comp;
    if x > 0 {
        search(x - 1, y, input, comp);
    }
    if x < input.len() - 1 {
        search(x + 1, y, input, comp);
    }
    if y > 0 {
        search(x, y - 1, input, comp);
    }
    if y < input[0].len() - 1 {
        search(x, y + 1, input, comp);
    }
    true
}
