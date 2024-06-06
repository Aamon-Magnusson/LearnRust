//use std::env;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //println!("The third index is: {}", args[2]);

    let height = 7;
    let column = 4;
    let row = 4;

    //let height: i32 = args[3];
    //let column: i32 = args[4];
    //let row: i32 = args[5];

    for row_index in 0..column {
        for height_index in 0..height {
            for _column_index in 0..row {
                for width_index in 0..height * 2 {
                    if row_index % 2 == 0 {
                        if width_index + 1 + height_index >= height && width_index + 1 - height_index <= height {
                            print!("*");
                        } else {
                            print!(" ");
                        }
                    } else {
                        if width_index + height - height_index <= height || width_index + height_index - height >= height {
                            print!("*");
                        } else {
                            print!(" ");
                        }
                    }
                }
            }
            println!("");
        }
    }

}
