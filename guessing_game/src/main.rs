use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries: u32 = 1;

    loop {
        println!("Turn {}: Please input your guess...", tries);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win in {} turns!!!!!!", tries);
                break;
            } 
        }
        println!("");
        tries += 1;
    }
}
