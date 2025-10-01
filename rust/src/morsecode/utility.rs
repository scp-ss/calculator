use std::io::{self, Write};
use std::process::Command;

pub fn clear_screen() {
    // Try Windows
    if Command::new("cmd").args(&["/C", "cls"]).status().is_ok() {
        return;
    }

    // Try Unix-based systems
    if Command::new("clear").status().is_ok() {
        return;
    }

    // Fallback
    println!("\n{}", "-".repeat(50));
}

pub fn prompt_choice() -> String {
    println!("\nChoose mode:");
    println!("1. Convert to Morse code");
    println!("2. Convert from Morse code");
    println!("3. Clear screen");
    println!("4. Exit");
    print!("Enter 1, 2, 3, or 4: ");
    io::stdout().flush().unwrap();

    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    mode.trim().to_string()
}


pub fn prompt_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
// 