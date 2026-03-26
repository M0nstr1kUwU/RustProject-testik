use std::io::{stdin, stdout, Write};
use rand::{rng, RngExt};
use std::cmp::Ordering::{Equal, Greater, Less};

fn main() {
    char_fill(50, '=');
    println!("1. Random Number\n2. Exit");
    char_fill(50, '=');

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match input.trim() {
            "0" => break,
            "1" => rand_num(),
            "2" => calculator(),
            _ => continue,
        }
    }
}

fn rand_num() {
    let secret = rng().random_range(1..=100);
    char_fill(50, '-');
    println!("Random Number: {}", secret);
    char_fill(50, '-');
    loop {
        print!("Your number: > ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn calculator() {
    char_fill(40, '-');
    println!("Calculator:\n1.History\n2.Clear\n3.Solve\n0.Exit");
    char_fill(40, '-');
    let v: Vec<String> = Vec::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        match input.trim() {
            "0" => break,
            "1" => {
                println!("History:");
                continue;
            },
            "2" => {
                println!("Clear:");
                continue;
            },
            "3" => {
                println!("Solve:");
            }
            _ => continue
        }
    }
}

fn char_fill(num: i32, char: char) {
    for _ in 0..num {
        print!("{}", char);
    }
    println!();
}