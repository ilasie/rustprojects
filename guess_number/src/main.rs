use std::{ io::stdin, cmp::Ordering };
use rand::random_range;

fn main() {
    println!("Please type an integer.");

    let key: u32 = random_range(1..=100);

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input)
            .expect("Error: Cannot read input.");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        match input.cmp(&key) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Win!");
                break;
            },
        }
    }
}
