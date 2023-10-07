use std::io::{stdin, Result as ResultIo};
use std::cmp::{Ordering};
use rand::{Rng, thread_rng};

fn main() {
    println!("Guess the number!");
    let secret_number = thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess");

        let input = read_line();

        let input: i32 = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(error) => {
                println!("Please enter a number: {}", error);
                continue;
            },
        };

        println!("Your guess: {}", input);

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn read_line() -> ResultIo<String> {
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(bytes) => {
            println!("Number of bytes read: {}", bytes);
        },
        Err(err) => {
            println!("Failed to read line\nError: {}", err);
        }
    }
    Ok(buffer)
}
