use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 默认immutable variable，即Java final

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing var guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ 是一个通配符值，本例中用来匹配所有 Err 值，不管其中有何种信息
        };

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // match 类似于Java switch
            Ordering::Less => println!("Too small!"), // arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
