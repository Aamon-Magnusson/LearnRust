use std::{thread,time};
use rand::Rng;

//const HEIGHT: usize = 47;
//const WIDTH: usize = 140;
const HEIGHT: usize = 35;
const WIDTH: usize = 85;

/*
 * Any live cell with fewer than two live neighbors dies, as if by underpopulation.
 * Any live cell with two or three live neighbors lives on to the next generation.
 * Any live cell with more than three live neighbors dies, as if by overpopulation
 * Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
 */

fn print_sep () {
    for _j in 0..WIDTH * 2 {
        print!("-");
    }
    println!("");
}

fn print_table (current: [[i32; WIDTH] ; HEIGHT]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if current[i][j] == 1 {
                print!("🟩");
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}

fn check_table (current: [[i32; WIDTH] ; HEIGHT]) -> [[i32; WIDTH] ; HEIGHT] {
    let mut new = [[0_i32; WIDTH] ; HEIGHT];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {

            let mut count: i32 = 0;

            if i != 0 {
                count += current[i- 1][j];
            }
            if j != 0 {
                count += current[i][j - 1];
            }
            if i != HEIGHT - 1 {
                count += current[i + 1][j];
            }
            if j != WIDTH - 1 {
                count += current[i][j + 1];
            }
            if i != 0 && j != 0 {
                count += current[i - 1][j - 1];
            }
            if i != 0 && j != WIDTH - 1 {
                count += current[i - 1][j + 1];
            }
            if i != HEIGHT - 1 && j != WIDTH - 1 {
                count += current[i + 1][j + 1];
            }
            if i != HEIGHT - 1 && j != 0 {
                count += current[i + 1][j - 1];
            }

            if current[i][j] == 1 {
                if count < 2 {
                    new[i][j] = 0;
                } else if count == 2 || count == 3 {
                    new[i][j] = 1;
                }else {
                    new[i][j] = 0;
                }
            } else {
                if count == 3 {
                    new[i][j] = 1;
                } else {
                    new[i][j] = 0;
                }
            }
        }
    }

    return new;
}

fn main() {
    let mut current = [[0; WIDTH] ; HEIGHT];

    let mut rng = rand::thread_rng();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            current[i][j] = rng.gen_range(0..=1);
        }
    }

    print_sep();
    print_table(current);
    print_sep();

    let mut generations = 0;
    let mut prev = [[0; WIDTH] ; HEIGHT];

    loop {
        generations += 1;

        let new = check_table(current);

        if current == new || prev == new {
            println!("The game of life survived for {generations} generations.");
            break;
        }

        let millis = time::Duration::from_millis(200);
        thread::sleep(millis);

        print_table(current);
        print_sep();

        prev = current;
        current = new;
    }
}
