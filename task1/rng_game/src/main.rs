use std::io;
use std::process::exit;
use rand::Rng;

fn main() {
    println!("\nGAME OF ODDS!\n");
    
    let winning_num:u32 = rand::rng().random_range(1..11);    
   
    loop {
        
    println!("Enter a guess for the chosen number");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Failed to enter the number!");
    let real_number:u32 = input_number.trim().parse().expect("Please Try again");
        
        if real_number == winning_num {
            println!("You won!");
            exit(1);
        }
        else if real_number < winning_num {
                println!("Your Guess is Smaller!\n");
                continue;
        }
        else {
            println!("Your Guess is Larger!\n");
            continue;
        }
        }   
    }
