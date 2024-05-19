use std::io;

//fn read_i32() -> i32 {
//    let line = io::stdin().lines().next().unwrap().unwrap();
//    line.parse().unwrap()
//}

fn read_u32() -> u32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}

fn main() {

    println!("Enter the first number: ");
    let num1 = read_u32();
    println!("Enter the second number: ");
    let num2 = read_u32();

    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} - {} = {}", num1, num2, num1 - num2);
    println!("{} * {} = {}", num1, num2, num1 * num2);
    println!("{} / {} = {}", num1, num2, num1 / num2);
    println!("{} % {} = {}", num1, num2, num1 % num2);
    println!("{} ^ {} = {}", num1, num2, u32::pow(num1, num2));
}
