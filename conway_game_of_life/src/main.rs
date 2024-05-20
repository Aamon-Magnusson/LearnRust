use std::{thread,time};

const HEIGHT: usize = 15;
const WIDTH: usize = 50;

/*
 * Any live cell with fewer than two live neighbors dies, as if by underpopulation.
 * Any live cell with two or three live neighbors lives on to the next generation.
 * Any live cell with more than three live neighbors dies, as if by overpopulation
 * Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
 */

fn print_sep () {
    for _j in 0..WIDTH {
        print!("-");
    }
    println!("");
}

fn main() {
    let mut current = [[0; WIDTH] ; HEIGHT];

    current[0] = [0,0,0,0,1,0,0,1,0,0,0,0,1,1,1,0,1,0,0,0,0,0,1,0,0,1,0,0,0,1,1,0,0,0,0,0,1,1,1,0,1,1,0,0,0,1,0,1,0,0];
    current[1] = [0,1,0,1,0,1,0,0,1,1,1,1,0,0,1,0,1,0,0,0,1,1,0,0,0,0,0,1,1,1,0,0,1,1,1,1,0,1,1,1,0,1,0,1,1,1,1,0,0,0];
    current[2] = [0,1,1,1,0,1,1,1,0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0,1,1,0,1,1,0,1,1,1,0,1,0,0,0,0,0,1,1,1,1,0,1,0,1,1,1];
    current[3] = [1,1,1,0,1,0,1,0,0,0,1,0,1,0,0,0,1,1,0,1,0,1,1,0,0,0,1,1,1,1,0,1,0,1,0,0,0,1,0,0,0,0,0,1,1,0,0,1,0,0];
    current[4] = [1,0,0,0,0,0,0,1,0,1,1,1,1,1,0,1,0,0,0,0,0,0,0,1,0,1,1,0,1,1,0,1,0,0,1,1,0,0,0,1,0,1,1,1,1,1,0,1,0,0];
    current[5] = [1,0,1,1,1,1,1,0,0,0,1,0,0,1,0,1,1,1,0,0,0,0,0,0,1,1,1,0,1,0,1,0,1,1,0,0,0,1,0,0,0,1,0,1,1,0,1,0,1,0];
    current[6] = [0,1,1,0,0,1,1,1,0,1,0,1,0,1,1,1,0,0,0,1,1,0,0,1,0,1,1,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,1,1,1,1,1,1,0,1];
    current[7] = [0,0,0,1,0,0,0,0,0,0,1,0,1,0,1,1,1,0,0,0,0,0,1,0,1,1,0,1,1,0,0,1,1,0,0,0,0,0,0,0,1,0,1,1,0,0,1,1,0,1];
    current[8] = [0,1,0,1,1,0,1,0,0,0,1,1,0,0,0,1,0,1,0,1,0,0,0,1,1,0,1,1,1,1,1,1,1,0,0,0,0,0,1,1,1,0,1,1,0,0,0,0,0,1];
    current[9] = [0,1,0,0,1,0,0,0,0,1,0,1,0,0,1,0,1,0,0,1,1,0,1,0,0,1,0,1,1,1,0,0,1,0,0,1,1,1,0,1,1,0,1,0,1,1,0,1,1,1];
    current[10] = [1,0,1,0,1,1,0,0,1,1,1,0,0,0,0,0,0,1,0,1,1,1,0,0,1,0,0,0,1,1,0,0,1,1,1,1,0,1,0,0,0,1,1,0,0,1,0,1,1,0];
    current[11] = [1,0,0,0,1,1,0,1,0,1,1,1,1,0,1,0,0,1,1,0,0,1,1,1,0,0,0,1,1,0,0,1,0,1,1,1,1,0,0,0,1,0,0,1,1,1,1,0,1,1];
    current[12] = [0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,1,0,1,1,0,0,0,0,1,1,1,1,1,1,0,1,1,0,0,0,1,1,1,0,0,0,0,0,1,1,1,0,0];
    current[13] = [0,0,1,0,0,1,1,0,0,1,0,1,0,0,1,1,0,1,1,0,1,1,1,0,0,1,0,1,1,0,1,1,0,0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,1,0];
    current[14] = [0,0,0,0,1,0,1,1,1,0,0,0,1,0,1,1,1,1,1,1,0,1,0,0,0,1,1,1,0,1,0,0,0,1,1,1,0,0,1,0,0,1,0,1,0,1,0,0,1,1];

    print_sep();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if current[i][j] == 1 {
                print!("🟩");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    print_sep();

    for _x in 0..15 {

        let mut new = [[0; WIDTH] ; HEIGHT];

        for i in 0..HEIGHT {
            for j in 0..WIDTH {

                let mut count: u32 = 0;

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

        let millis = time::Duration::from_millis(200);
        thread::sleep(millis);

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if new[i][j] == 1 {
                    print!("🟩");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        print_sep();
        current = new;
    }
}
