/// test cases
#[cfg(test)]
mod tests {
    use crate::daily::{round_two_decimals, format_date, fetch_stock_data_daily, fetch_stock_data_historical};

    #[test]
    fn test_round_two_decimals() {
        assert_eq!(round_two_decimals(3.14159), 3.14);
        assert_eq!(round_two_decimals(2.71828), 2.72);
    }

    #[test]
    fn test_format_date() {
        assert_eq!(format_date("2024-06-27").unwrap(), "June-27-2024");
        assert!(format_date("invalid-date").is_err());
    }

    #[test]
    fn test_fetch_stock_data_daily() {
        let api_key = "YOUR_API_KEY";
        let symbol = "AAPL";
        let result = fetch_stock_data_daily(api_key, symbol);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fetch_stock_data_historical() {
        let api_key = "YOUR_API_KEY";
        let symbol = "AAPL";
        let result = fetch_stock_data_historical(api_key, symbol);
        assert!(result.is_ok());
    }
}
