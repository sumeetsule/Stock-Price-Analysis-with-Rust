use std::io;

/// Function to get user choice for time frame
pub fn user_choice() -> String {
    println!(
        "Enter the time frequency you want to look for: 
            \n Select from the following: 
            \n 1. Year 2024
            \n 2. Historical Data
            \n Enter your choice:"
            );

    let mut freq_choice = String::new();
    io::stdin().read_line(&mut freq_choice).expect("Failed");
    freq_choice.trim().to_string()
}


/// Function to get symbol for which the user wants to see the stock data
pub fn user_symbol() -> String {
    println!("Enter the stock symbol you want to see:");

    let mut symbol_choice = String::new();
    io::stdin().read_line(&mut symbol_choice).expect("Failed");
    symbol_choice.trim().to_string()
}