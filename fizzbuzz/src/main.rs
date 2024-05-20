fn main() {
    let mut count: u32 = 1;

    loop {

        let mut output = String::new();

        if count % 3 == 0 {
            output.push_str("Fizz");
        }
        if count % 5 == 0 {
            output.push_str("Buzz");
        }

        if output.is_empty() {
            println!("{count}");
        } else {
            println!("{output}");
        }

        count += 1;
        if count == 100 {
            break;
        }
    }
}
