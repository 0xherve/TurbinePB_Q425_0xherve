//Building the Hello World program in rust 
// Adding the feature of asking your name first 
// The program will be greating you personally 

use std::io;

fn main() {
    let mut name = String::new();
    
    println!("Initializing the Hello World Program!");
    println!("What is your name:");
    io::stdin().read_line(& mut name).expect("Failed to input the name. Please try again.");    
    
    println!("Hello {}, Let's say Hello World together!", name.trim());
    println!();
    
    
    println!("Hello, world!");
}
