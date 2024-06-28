mod user;
mod daily;
mod test;

use crate::user::{user_choice,user_symbol};
use crate::daily::{stock_data_daily,stock_data_historical};


fn main() {
    let symbol = user_symbol();
    let user_choice = user_choice();

    if user_choice == "Year 2024" {
        
        println!("Here is the 2024 stock data!");
        stock_data_daily(&symbol);

    } else if user_choice == "Historical Data" {
        
        println!("Here is the historical stock data!");
        stock_data_historical(&symbol);

    } else {
        println!("Sorry could'nt process your input");
    }

}