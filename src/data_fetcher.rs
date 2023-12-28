use reqwest;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;

//
static API_KEY: &str = "935X1MVA8K04G0P2";

pub struct DataFetcher {
    pub base_url: String,
    pub api_key: String,
}

pub fn new() -> DataFetcher {
    DataFetcher {
        base_url: String::from("https://www.alphavantage.co/query?"),
        api_key: String::from(API_KEY),
    }
}
impl DataFetcher {
    // Get the last closing price of the symbol
    pub async fn get_current_price(&self, symbol: &str) -> Result<f64, Box<dyn Error>> {
        let url = format!(
            "{}function=TIME_SERIES_DAILY&symbol={}&outputsize={}&apikey={}",
            self.base_url, symbol, "compact", self.api_key
        );

        let response = reqwest::get(&url).await?;
        let data = response.json::<Value>().await?;

        if let Some(time_series) = data["Time Series (Daily)"].as_object() {
            let mut dates: Vec<&str> = time_series.keys().map(AsRef::as_ref).collect();
            dates.sort();
            if let Some(&latest_date) = dates.last() {
                if let Some(close_price) = time_series[latest_date]["4. close"].as_str() {
                    return close_price.parse::<f64>().map_err(|e| e.into());
                }
            }
        }

        Err("Unable to find the latest close price".into())
    }
}

//Want to fetch data for the current price of an action
//Want to fetch data for
