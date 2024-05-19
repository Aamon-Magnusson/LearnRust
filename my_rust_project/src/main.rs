//use std::io;
use rand::Rng;

//const TEST: i32 = 15;

//fn test_function(first_param: i32, second_param: i32) {
//    println!("{}", first_param + second_param)
//}

/// This is the main function :)
fn main() {
    // Stings &str
    //println!("Hello, world!");
    //println!("Stings must be in double quotes");

    // Integers
    // u is unsigned
    // i is signed
    //println!("Max size of a u32 {}", u32::MAX);
    //println!("Max size of a i32 {}", i32::MAX);
    //println!("Max size of a u64 {}", u64::MAX);
    //println!("Max size of a i64 {}", i64::MAX);

    // Floats
    // f32, f64
    //println!("Max size of a f32 {}", f32::MAX);
    //println!("Max size of a f64 {}", f64::MAX);

    // Boolean bool

    // character - char - 4 bytes
    //println!("{}", 'A');

    //let mut test_var = "Hello world";
    //println!("{}", test_var);
    //test_var = "Test";
    //println!("{}", test_var);

    //let x: i32 = 5;
    //let y: i32 = 6;
    //println!("{}", x + y);

    //const NUMBER: i32 = 17;
    //println!("{}", NUMBER + x);

    //println!("{}", TEST);

    //test_function(14, 16);

    //Suffixes  - specifies the type a numeric literal
    //let x = 42_u32;
    //let y = 12u32;
    //println!("{}", x + y);

    // compound types
    // Tuple - up to 12 values - of different data types

    //let student = ("Aamon", 'A', 3.97);
    //let (name, grade, gpa) = student;

    //println!("{:?}", student);
    //println!("{name} {grade} {gpa}");

    //// arrays - up to 32 values - similar data types
    //let students = ["Heath", "Bob", "Linda"];
    //println!("{}", students[0]);

    //// Slices
    //let mut arr = [1, 2, 3, 4, 5];
    //let slice = &mut arr[1..3];
    //slice[0] = 14;
    //println!("{:?}",slice);
    //println!("{:?}",arr);

    // String and &str
    // str - String slice, &str burrowed string slice - cannot be modified
    // String - can be a modified
    // &str - subset of String
    //let name = "Aamon".tostring();

    //let mut name: String = String::from("Aamon");
    //name.push_str("Aamon");
    //name.push_str(" test");
    //println!("{}", name);

    //println!("Hello, world!");
    //println!("Hello, \
    //         world!");
    //println!("\"Hello, world!\"");
    //println!("\tHello, world!");
    //println!("\nHello, world!");
    //println!("{}", concat!("Hello", "World"));

    //println!("How goes there?");
    //let mut name = String::new();
    //io::stdin().read_line(&mut name);
    //println!("{}", name);
    //let enter = "You may enter.";
    //println!("Hello there {}. {}", name.trim_end(), enter);

    //let x: u32 = 10;
    //let y: u32 = 3;
    //let x_float: f64 = x as f64;
    //let y_float: f64 = y as f64;

    //println!("{} + {} = {}", x , y, x + y);
    //println!("{} - {} = {}", x , y, x - y);
    //println!("{} * {} = {}", x , y, x * y);
    //println!("{} / {} = {}", x , y, x_float / y_float);
    //println!("{} % {} = {}", x , y, x % y);
    //println!("{} ^ {} = {}", x , y, u32::pow(x,y));

    let x = rand::thread_rng().gen_range(1..101);
    println!("{x}");

}
