use std::io::{self, Write};

fn main() {

    // Main loop
    loop {
        println!("\nRust Image Processor [Select an option]");
        println!("Current Image:");
        println!("[1] Upload an image");
        println!("[2] Resize image");
        println!("[3] Blur image");
        println!("[4] Convert to grayscale");
        println!("[5] Exit");
        println!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {

            // Call functions for each action
            "1" => break,
            "2" => break,
            "3" => break,
            "4" => break,
            "5" => {
                println!("\nExiting...\n");
                break;
            }
            _ => println!("\nInvalid option. Please try again."),
        }

    }

}
