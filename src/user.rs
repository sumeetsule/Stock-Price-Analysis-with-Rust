use std::io;

/// Prompts the user to enter a time frame choice and returns their input as a string.
///
/// # Returns
///
/// A `String` containing the user's choice. The choices are:
/// 
/// - "1" for Year 2024
/// - "2" for Historical Data
///
/// # Examples
///
/// ```
/// let choice = user_choice();
/// println!("User selected: {}", choice);
/// ```
///
/// # Panics
///
/// This function will panic if it fails to read the input from the standard input stream.
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

/// Prompts the user to enter a stock symbol and returns their input as a string.
///
/// # Returns
///
/// A `String` containing the stock symbol entered by the user.
///
/// # Examples
///
/// ```
/// let symbol = user_symbol();
/// println!("User entered symbol: {}", symbol);
/// ```
///
/// # Panics
///
/// This function will panic if it fails to read the input from the standard input stream.
pub fn user_symbol() -> String {
    println!("Enter the stock symbol you want to see:");

    let mut symbol_choice = String::new();
    io::stdin().read_line(&mut symbol_choice).expect("Failed");
    symbol_choice.trim().to_string()
}
