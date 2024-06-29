// src/daily.rs

use chrono::format::ParseError;
use chrono::NaiveDate;
use polars::prelude::*;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use postgres::{Client as PgClient,NoTls};


/// Struct for stock data
#[derive(Debug, Deserialize)]
pub struct TimeSeries {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}


/// Struct for API
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "Time Series (Daily)")]
    pub time_series: HashMap<String, TimeSeries>,
}


/// Function to fetch the stock data from Alpha Vantage
pub fn fetch_stock_data_daily(api_key: &str, symbol: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!(
        "https:///www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}",
        symbol, api_key
    );

    let client = Client::new();
    let response = client.get(&url).send()?.json::<ApiResponse>()?;
    Ok(response)
}


/// Function to pass API key and stock symbol
pub fn stock_data_daily(symbol: &str) {
    let api_key = "YPHKF6BPQ68DT14O"; 
    match fetch_stock_data_daily(api_key, symbol) {
        Ok(data) => {

            // Get the data and sort them according to dates in ascending order
            let mut dates: Vec<&String> = data.time_series.keys().collect();
            dates.sort(); 
            
            // Create vector to store data
            let mut date_vec = Vec::new();
            let mut open_vec = Vec::new();
            let mut high_vec = Vec::new();
            let mut low_vec = Vec::new();
            let mut close_vec = Vec::new();
            let mut volume_vec = Vec::new();

            
            // Format the data
            for date in dates {
                let series = &data.time_series[date];
                date_vec.push(format_date(date).unwrap_or_else(|_| date.to_string()));
                open_vec.push(round_two_decimals(series.open.parse::<f64>().unwrap()));
                high_vec.push(round_two_decimals(series.high.parse::<f64>().unwrap()));
                low_vec.push(round_two_decimals(series.low.parse::<f64>().unwrap()));
                close_vec.push(round_two_decimals(series.close.parse::<f64>().unwrap()));
                volume_vec.push(series.volume.parse::<i64>().unwrap());
            }


            // Create series objects will then be used to create dataframe
            let date_series = Series::new("Date", date_vec);
            let open_series = Series::new("Open", open_vec);
            let high_series = Series::new("High", high_vec);
            let low_series = Series::new("Low", low_vec);
            let close_series = Series::new("Close", close_vec);
            let volume_series = Series::new("Volume", volume_vec);


            // Create the dataframe using polars
            let df = DataFrame::new(vec![
                date_series,
                open_series,
                high_series,
                low_series,
                close_series,
                volume_series,
            ])
            .unwrap();

            println!("{:?}", df);
            println!("Number of rows: {}", df.height());
            println!("Number of columns: {}", df.width());
            

            // Calculate mean for the values
            let open_mean = df.column("Open").unwrap().f64().unwrap().mean().unwrap();
            let high_mean = df.column("High").unwrap().f64().unwrap().mean().unwrap();
            let low_mean = df.column("Low").unwrap().f64().unwrap().mean().unwrap();
            let close_mean = df.column("Close").unwrap().f64().unwrap().mean().unwrap();
            let volume_mean = df.column("Volume").unwrap().i64().unwrap().mean().unwrap();

            println!("Mean Open: {:.2}", open_mean);
            println!("Mean High: {:.2}", high_mean);
            println!("Mean Low: {:.2}", low_mean);
            println!("Mean Close: {:.2}", close_mean);
            println!("Mean Volume: {:.2}", volume_mean);

        }
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}


/// Function to fetch the stock data from Alpha Vantage
pub fn fetch_stock_data_historical(api_key: &str,symbol: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!(
        "https:///www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&outputsize=full&apikey={}",
        symbol, api_key
    );

    let client = Client::new();
    let response = client.get(&url).send()?.json::<ApiResponse>()?;
    Ok(response)
}


/// Function to pass API key and stock symbol
pub fn stock_data_historical(symbol: &str) {
    let api_key = "YPHKF6BPQ68DT14O"; 
    match fetch_stock_data_historical(api_key, symbol) {
        Ok(data) => {

            // Get the data and sort them according to dates in ascending order
            let mut dates: Vec<&String> = data.time_series.keys().collect();
            dates.sort(); 


            // Create vector to store data
            let mut date_vec = Vec::new();
            let mut open_vec = Vec::new();
            let mut high_vec = Vec::new();
            let mut low_vec = Vec::new();
            let mut close_vec = Vec::new();
            let mut volume_vec = Vec::new();


            // Format the data
            for date in dates {
                let series = &data.time_series[date];
                date_vec.push(format_date(date).unwrap_or_else(|_| date.to_string()));
                open_vec.push(round_two_decimals(series.open.parse::<f64>().unwrap()));
                high_vec.push(round_two_decimals(series.high.parse::<f64>().unwrap()));
                low_vec.push(round_two_decimals(series.low.parse::<f64>().unwrap()));
                close_vec.push(round_two_decimals(series.close.parse::<f64>().unwrap()));
                volume_vec.push(series.volume.parse::<i64>().unwrap());
            }



            let date_series = Series::new("Date", date_vec);
            let open_series = Series::new("Open", open_vec);
            let high_series = Series::new("High", high_vec);
            let low_series = Series::new("Low", low_vec);
            let close_series = Series::new("Close", close_vec);
            let volume_series = Series::new("Volume", volume_vec);


            // Create the dataframe using polars
            let df = DataFrame::new(vec![
                date_series,
                open_series,
                high_series,
                low_series,
                close_series,
                volume_series,
            ])
            .unwrap();

            println!("{:?}", df);
            println!("Number of rows: {}", df.height());
            println!("Number of columns: {}", df.width());
            

            // Calculate mean for the values
            let open_mean = df.column("Open").unwrap().f64().unwrap().mean().unwrap();
            let high_mean = df.column("High").unwrap().f64().unwrap().mean().unwrap();
            let low_mean = df.column("Low").unwrap().f64().unwrap().mean().unwrap();
            let close_mean = df.column("Close").unwrap().f64().unwrap().mean().unwrap();
            let volume_mean = df.column("Volume").unwrap().i64().unwrap().mean().unwrap();

            println!("Mean Open: {:.2}", open_mean);
            println!("Mean High: {:.2}", high_mean);
            println!("Mean Low: {:.2}", low_mean);
            println!("Mean Close: {:.2}", close_mean);
            println!("Mean Volume: {:.2}", volume_mean);
            
            
            // Match function to store the data
            match save_to_db(&df) {
                Ok(_) => println!("Data saved to database successfully."),
                Err(e) => eprintln!("Error saving data to database: {}", e),
            }
        }
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}


/// Function that changes the date from YYYY-MM-DD format to MM-DD-YYYY only for the output
pub fn format_date(date: &str) -> Result<String, ParseError> {
    let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
    Ok(parsed_date.format("%B-%d-%Y").to_string())
}


/// Rounds up the value to have only two digits after decimal point
pub fn round_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}


/// Function that stores the data in PostgreSQL database named "Rustproject"
fn save_to_db(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    let mut client = PgClient::connect("host=localhost user=postgres password=sumeet dbname=Rustproject", NoTls)?;

    let date_series = df.column("Date")?;
    let open_series = df.column("Open")?;
    let high_series = df.column("High")?;
    let low_series = df.column("Low")?;
    let close_series = df.column("Close")?;
    let volume_series = df.column("Volume")?;


    // Loop through each value and store in the database
    for i in 0..df.height() {
        let date_str: &str = date_series.str()?.get(i).ok_or("Invalid date")?;
        let open: f64 = open_series.f64()?.get(i).ok_or("Invalid open value")?;
        let high: f64 = high_series.f64()?.get(i).ok_or("Invalid high value")?;
        let low: f64 = low_series.f64()?.get(i).ok_or("Invalid low value")?;
        let close: f64 = close_series.f64()?.get(i).ok_or("Invalid close value")?;
        let volume: i64 = volume_series.i64()?.get(i).ok_or("Invalid volume value")?;

        
        let date = NaiveDate::parse_from_str(date_str, "%B-%d-%Y")?;


        // Query to insert the data into table named "stock_data"
        client.execute(
            "INSERT INTO stock_data (date, open, high, low, close, volume) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&date, &open, &high, &low, &close, &volume],
        )?;
    }

    Ok(())
}