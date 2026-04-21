// https://doc.rust-lang.org/book/ch03-05-control-flow.html

use std::io::{stdin, stdout, Write};
use rand::{rng, RngExt};
use std::cmp::Ordering::{Equal, Greater, Less};

fn main() {
    char_main_fill(30, '~');
    println!("1. Random Number\n2. Calculator\n0. Exit");
    char_main_fill(30, '~');

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match input.trim() {
            "0" => break,
            "1" => char_fill(30, '=', rand_num),
            "2" => char_fill(30, '-', calculator),
            _ => continue,
        }
    }
}

fn rand_num() {
    let secret = rng().random_range(1..=100);
    println!("Random Number: {secret}");
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
    println!("Calculator:\n1.History\n2.Clear\n3.Solve\n0.Exit");
    let mut history: Vec<String> = Vec::new();

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
                if history.is_empty() {
                    println!("  (empty)");
                } else {
                    for (i, entry) in history.iter().enumerate() {
                        println!("{}. {entry}", i+1);
                    }
                }
            },
            "2" => {
                history.clear();
                println!("History cleared.");
            },
            "3" => {
                println!("Solve:");
                loop {
                    print!("Your solve: > ");
                    stdout().flush().unwrap();

                    let mut expr = String::new();
                    stdin()
                        .read_line(&mut expr)
                        .expect("Failed to read line!");
                    let expr = expr.trim();

                    if expr.is_empty() {
                        continue;
                    }

                    let parts: Vec<&str> = expr.split_whitespace().collect();
                    if parts.len() != 3 {
                        println!("Invalid format! Use: number operator number");
                        continue;
                    }

                    let x: f64 = match parts[0].parse() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Invalid number: {}", parts[0]);
                            continue;
                        }
                    };

                    let op = parts[1];

                    let y: f64 = match parts[2].parse() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Invalid number: {}", parts[2]);
                            continue;
                        }
                    };

                    let result: f64 = match op {
                        "+" => x + y,
                        "-" => x - y,
                        "*" => x * y,
                        "/" => {
                            if y == 0.0 {
                                println!("Division by zero!");
                                continue;
                            }
                            x / y
                        }
                        "%" => x % y,
                        _ => {
                            println!("Unknown operator: {op}");
                            continue;
                        }
                    };

                    let entry: String = format!("{x} {op} {y} = {result}");
                    println!("Result: {result}");
                    history.push(entry);

                    if history.len() > 10 {
                        history.remove(0);
                    }
                    break;
                }
            }
            _ => continue
        }
    }
}

fn char_fill<F>(num: usize, char: char, func: F)
where
    F: FnOnce(),
{
    for _ in 0..num {
        print!("{}", char);
    }
    println!();

    func();

    for _ in 0..num {
        print!("{}", char);
    }
    println!();
}

fn char_main_fill(num: usize, char: char) {
    for _ in 0..num {
        print!("{}", char);
    }
    println!();
}