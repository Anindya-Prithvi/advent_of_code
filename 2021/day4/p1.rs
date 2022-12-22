use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let allnumandboards = input.split("\n\n").collect::<Vec<&str>>();
    // all boards are given to be 5x5
    let totalboards = allnumandboards.len() - 1;
    let mut boards = vec![vec![vec![0; 5]; 5]; totalboards];
    let mut visitedboards = vec![vec![vec![false; 5]; 5]; totalboards];
    //populate the boards
    for i in 0..totalboards {
        let board = allnumandboards[i + 1].split("\n").collect::<Vec<&str>>();
        for j in 0..5 {
            let row = board[j]
                .split(" ")
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();
            for k in 0..5 {
                boards[i][j][k] = row[k].parse::<i32>().unwrap();
            }
        }
    }
    // start the game
    let drawnnumbers = allnumandboards[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    for x in drawnnumbers {
        for i in 0..totalboards {
            for row in 0..5 {
                for col in 0..5 {
                    if boards[i][row][col] == x {
                        visitedboards[i][row][col] = true;
                    }
                }
            }
        }
        if win_please(&boards, &visitedboards, x) {
            break;
        }
    }
}

fn win_please(boards: &Vec<Vec<Vec<i32>>>, visitedboards: &Vec<Vec<Vec<bool>>>, x: i32) -> bool {
    for i in 0..boards.len() {
        for row in 0..5 {
            //fold and check
            if visitedboards[i][row].iter().fold(true, |acc, x| acc && *x) {
                // print required
                println!("worked!!on row");
                let mut sumunm = 0;
                for a in 0..5 {
                    for b in 0..5 {
                        if !visitedboards[i][a][b] {
                            sumunm += boards[i][a][b];
                        }
                    }
                }
                println!("{}", sumunm * x);
                return true;
            }
        }
        for col in 0..5 {
            if visitedboards[i].iter().fold(true, |acc, x| acc && x[col]) {
                println!("worked!!on col");
                let mut sumunm = 0;
                for a in 0..5 {
                    for b in 0..5 {
                        if !visitedboards[i][a][b] {
                            sumunm += boards[i][a][b];
                        }
                    }
                }
                println!("{}", sumunm * x);
                return true;
            }
        }
    }
    false
}
