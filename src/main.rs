use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Guessing Game!");
    println!("Please guess a number between 0 and 256: ");

    let mut rng = rand::thread_rng();
    let target: u8 = rng.gen();

    let mut input = String::new();

    while let Ok(_) = io::stdin().read_line(&mut input) {
        match input.trim().parse::<u8>() {
            Ok(value) => {
                if value == target{
                    println!("Correct!");
                    return;
                }
                else if value > target{
                    println!("Lower!");
                }
                else {
                    println!("Higher!")
                }
            },
            Err(err) => println!("Someding wong: {}", err)
        }
        input.clear();
    }

}
