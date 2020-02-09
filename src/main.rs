use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\n\nWelcome to the silly guess the number game!\n");

    let secret_number = rand::thread_rng().gen_range(1, 101); // Random number between 1 and 100.

    loop {
	println!("Please input your guess (between 1 and 100) below:");

	let mut guess = String::new(); // Take in a string which the user enters.

	io::stdin().read_line(&mut guess)
	    .expect("Failed to read line"); // Only triggered with an operating system error.

	let guess: u32 = match guess.trim().parse() { // Shadowing the guess variable and converting it to an unsigned 32 bit integer.
	    Ok(num) => num,
	    Err(_) => {
		println!("Ya done goofed! Please enter a positive number between 1 and 100.\n"); // Old memes die hard, I guess.
		continue;
	    }
	};

	if guess > 100 {
	    println!("You can't pick a number larger than 100, you idiot!\n"); // Harsh, but true.
	} else if guess < 1 {
	    println!("You can't pick a number smaller than 1, you idiot!\n");
	} else 	{
	    println!("You guessed: {}", guess);

	    match guess.cmp(&secret_number) { // Comparison between the user input and the random number generated.

		Ordering::Less => println!("The number you guessed is too small!\n"),
		Ordering::Greater => println!("The number you guessed is too big!\n"),
		Ordering::Equal => {
		    println!("You guessed the correct number. You win!\n");
		    break;
		}

	    }
	}
    }
}
