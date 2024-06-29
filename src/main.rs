mod user;
mod daily;
mod test;

use crate::user::{user_choice, user_symbol};
use crate::daily::{stock_data_daily, stock_data_historical};

/// Main function that prompts the user for a stock symbol and time frame, 
/// then displays the appropriate stock data.
pub fn main() {
    let symbol = user_symbol();
    let user_choice = user_choice();

    match user_choice.as_str() {
        "Year 2024" => {
            println!("Here is the 2024 stock data!");
            stock_data_daily(&symbol);
        },
        "Historical Data" => {
            println!("Here is the historical stock data!");
            stock_data_historical(&symbol);
        },
        _ => println!("Sorry, couldn't process your input"),
    }
}