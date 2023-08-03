use core::cmp::Ordering;
use rand::prelude::*;
use std::io::{stdin, stdout, Write};

fn main() {
    let x: u8 = random();
    let mut attempts = 0;

    print!("Guess a number between 1 and 256! ");

    loop {
        let _ = stdout().flush();
        let mut y = String::new();
        attempts += 1;

        stdin()
            .read_line(&mut y)
            .expect("Please input a valid number! ");

        if y.trim().contains("q") {
            println!("Quitting...");
            println!("The number was {} ;)", x);
            break;
        };

        let ans: u8 = match y.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                print!("Please input a valid number! ");
                continue;
            }
        };

        match ans.cmp(&x) {
            Ordering::Less => print!("Too small! "),
            Ordering::Greater => print!("Too big! "),
            Ordering::Equal => {
                println!(
                    "Congratulations!\nYou guessed it in {} attempts...",
                    attempts
                );
                break;
            }
        }
    }
}
