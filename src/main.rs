use std::io; // input/output library
use rand::Rng; // Generation iof random numbers
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line!"); // accept user input and returns a Result type 

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // trim => removes whitespace from beginning and end of input.. parse => switches variables from one type to another. in this case a u32 bit integer.

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number you guessed is too small!"),
            Ordering::Greater => println!("The number you guessed is too big!"),
            Ordering::Equal => {
                println!("You guessed the right number! You win!!!");
                break;
            }
        };
    }
    
}
