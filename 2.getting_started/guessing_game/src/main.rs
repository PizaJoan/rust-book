use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_rand = rand::thread_rng().gen_range(1..=100);
    println!("Secret is {secret_rand}");
    
    loop {
        println!("Guess the number!");
        println!("Please input your guess");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_rand) {
            Ordering::Less => println!("Low!"),
            Ordering::Greater => println!("High!"),
            Ordering::Equal =>{
                println!("Correct!");
                break;
            }
        }
    }
}
