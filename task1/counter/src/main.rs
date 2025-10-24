use std::io;

fn main() {
    println!("Rust Counter Project!\n");
    
    let mut counter = 0;
     println!("Enter the number you want to count up to:");
     
     let mut enter_number = String::new();
     io::stdin().read_line(&mut enter_number).expect("Error entering the Number!");
     
     let real_number:i64 = enter_number.trim().parse().expect("Please try entering a proper number");
     
     if real_number == 0 {
         println!("\nPlease enter a different number from 0!\n");
         std::process::exit(1);
     }
     
     println!("\nMaking a counter up to {real_number}\n");
     
    while counter <= real_number {
        println!("{counter}");
        counter += 1;
    }
    
    println!("\nThe END!")
}
