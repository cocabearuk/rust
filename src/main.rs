use ferris_says::say;
use std::io;
use std::{io::{stdout, BufWriter}};
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("What would you like to do?");

    println!("Enter 1 for Ferris says or 2 for guessing game:");

    let res = read_line("Failed to read line");

    if res.trim() == "1"
    {
        let stdout = stdout();
        let message = String::from("Hello youse");
        let width  = message.chars().count();
    
        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width, &mut writer).unwrap();
    }
    else {

        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("The secret number is: {}", secret_number);

        println!("Enter your guess:");

        let guess:String = read_line("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
    

}

fn read_line(err: &str) -> String{
    let mut res= String::new();
    io::stdin()
    .read_line(&mut res)
    .expect(&err);
    return res;
}