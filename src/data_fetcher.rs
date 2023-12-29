use chrono::NaiveDate;
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;

static API_KEY: &str = "935X1MVA8K04G0P2";
type Years = f64;

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
    // TODO: I guess I need to create a function to get the data during a speific period
    // The time period is in years
    async fn get_historical_prices(
        &self,
        symbol: &str,
        until_date: Option<&NaiveDate>,
    ) -> Result<HashMap<NaiveDate, DailyData>, Box<dyn Error>> {
        let url = format!(
            "{}function=TIME_SERIES_DAILY&symbol={}&outputsize={}&apikey={}",
            self.base_url, symbol, "full", self.api_key
        );

        let response = reqwest::get(&url).await?;
        let data = response.json::<ApiResponse>().await?;

        match until_date {
            Some(until_date) => {
                return Ok(data
                    .time_series
                    .into_iter()
                    .filter(|(date, _)| date < until_date)
                    .collect())
            }
            None => return Ok(data.time_series),
        }
    }

    // Get the last closing price of the symbol
    pub async fn get_last_price(&self, symbol: &str) -> Result<f64, Box<dyn Error>> {
        let prices = self.get_historical_prices(symbol, None).await?;
        let max_date_metrics = prices.iter().max_by_key(|(&date, _)| date).unwrap().1;
        return Ok(max_date_metrics.close);
    }
    //pub async fn compute_historical_volatility(&self, symbol: &str) -> Result<f64, Box<dyn Error>> {

    //}
}

//Different JSON
#[derive(Deserialize)]
struct ApiResponse {
    #[serde(rename = "Meta Data")]
    meta_data: MetaData,
    #[serde(rename = "Time Series (Daily)")]
    time_series: HashMap<NaiveDate, DailyData>,
}

#[derive(Deserialize)]
struct MetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "4. Output Size")]
    output_size: String,
    #[serde(rename = "5. Time Zone")]
    time_zone: String,
}

#[derive(Deserialize)]
struct DailyData {
    #[serde(rename = "1. open")]
    open: f64,
    #[serde(rename = "2. high")]
    high: f64,
    #[serde(rename = "3. low")]
    low: f64,
    #[serde(rename = "4. close")]
    close: f64,
    #[serde(rename = "5. volume")]
    volume: f64,
}
