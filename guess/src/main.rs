extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guessresult = false;
    let mut guesstime = 0;
    while !guessresult {
        if guesstime >= 10{
            break;
        }
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail to read line");

        let guess: u32 = guess.trim().parse().expect("Need a number!");

        println!("Your guess is: {}, the secret number is: {}", guess, secret_number);

        match guess.cmp(&secret_number){
            Ordering::Less    => {guesstime += 1;println!("Too Small!")},
            Ordering::Equal   => guessresult = true,//println!("You Win!"),
            Ordering::Greater => {guesstime += 1;println!("Too Large!")},
        };
    }
    if guesstime >= 10 {
        println!("You guess too much times!");
    }else{
        println!("You Win!");
    }
}
