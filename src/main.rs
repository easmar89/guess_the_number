use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=====================");
    println!("GUESS THE NUMBER GAME");
    println!("=====================");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Please choose a number betwen 1 and 100:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..try again: "),
            Ordering::Greater => println!("Too big...try again: "),
            Ordering::Equal => {
                println!("YES! Congrats");
                break;
            }
        }
    }
}
