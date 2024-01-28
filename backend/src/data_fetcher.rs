use chrono::Month;
use chrono::{Datelike, Duration, NaiveDate};
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
    pub async fn get_prices_of_last_year(
        &self,
        symbol: &str,
    ) -> Result<(Vec<u32>, Vec<String>), Box<dyn Error>> {
        let today = chrono::offset::Local::today().naive_local();
        let one_year_ago = today - chrono::Duration::days(365);
        let historical_prices = self.get_historical_prices(symbol, Some(&today)).await?;
        let mut monthly_prices: HashMap<u32, (NaiveDate, String)> = HashMap::new();

        for (date, data) in historical_prices
            .into_iter()
            .filter(|(date, _)| *date > one_year_ago && *date <= today)
        {
            match monthly_prices.get(&date.month()) {
                Some((dateSaved, val)) => {
                    if date < *dateSaved {
                        monthly_prices.insert(date.month(), (date, data.close));
                    }
                }
                None => _ = monthly_prices.insert(date.month(), (date, data.close)),
            };
        }

        Ok((
            monthly_prices.iter().map(|(&month, _)| month).collect(),
            monthly_prices
                .iter()
                .map(|(_, price)| price.1.clone())
                .collect(),
        ))
    }

    pub async fn compute_historical_volatility(
        &self,
        symbol: &str,
        until_date: Option<&NaiveDate>,
    ) -> Result<f64, Box<dyn Error>> {
        let prices = self.get_historical_prices(symbol, until_date).await?;
        if prices.len() < 2 {
            return Err("Not enough data to compute volatility".into());
        }

        let mut price_vec: Vec<_> = prices.iter().collect();
        price_vec.sort_by_key(|&(date, _)| date);

        let returns: Vec<f64> = price_vec
            .windows(2)
            .filter_map(|price| {
                let prev_price = price[0].1.close.parse::<f64>().ok()?;
                let curr_price = price[1].1.close.parse::<f64>().ok()?;
                let return_val = curr_price - prev_price / prev_price;
                Some(return_val)
            })
            .collect();

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let squared_diff_sum = returns
            .iter()
            .map(|&return_val| (return_val - mean_return).powi(2))
            .sum::<f64>();
        let variance = squared_diff_sum / returns.len() as f64;
        let volatility = variance.sqrt();

        Ok(volatility)
    }
    pub async fn get_best_matches(
        &self,
        symbol: &str,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let url = format!(
            "{}function=SYMBOL_SEARCH&keywords={}&apikey={}",
            self.base_url, symbol, self.api_key
        );

        let response = reqwest::get(&url).await?;
        let api_response_parsed: ApiResponseGetMatches = match response.json().await {
            Ok(matches) => matches,
            Err(err) => {
                eprintln!("Error parsing JSON response: {}", err);
                return Err(err.into());
            }
        };

        Ok(api_response_parsed
            .best_matches
            .into_iter()
            .map(|item| item.symbol)
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_best_matches() {
        let data_fetcher = DataFetcher {
            base_url: String::from("https://www.alphavantage.co/query?"),
            api_key: String::from(API_KEY),
        };

        let symbol = "AAPL";
        let expected_matches = vec!["AAPL", "AAPL.US"];

        let matches = data_fetcher.get_best_matches(symbol).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_prices_of_last_year() {
        let data_fetcher = new();
        let symbol = "AAPL";

        let result = data_fetcher.get_prices_of_last_year(symbol).await;

        assert!(result.is_ok());
        let (months, prices) = result.unwrap();

        // Assert that the months and prices vectors have the same length
        assert_eq!(months.len(), prices.len());

        // Assert that the months vector contains unique values
        let unique_months: Vec<_> = months.iter().collect();
        assert_eq!(months.len(), unique_months.len());

        // Assert that the prices vector is not empty
        assert!(!prices.is_empty());

        // Display the results
        println!("Months: {:?}", months);
        println!("Prices: {:?}", prices);
    }
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
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}
#[derive(Deserialize, Debug)]
struct ApiResponseGetMatches {
    #[serde(rename = "bestMatches")]
    best_matches: Vec<BestMatch>,
}

#[derive(Deserialize, Debug)]
struct BestMatch {
    #[serde(rename = "1. symbol")]
    symbol: String,
    #[serde(rename = "2. name")]
    name: String,
    #[serde(rename = "3. type")]
    _type: String, // Prefix with underscore because `type` is a reserved keyword in Rust
    #[serde(rename = "4. region")]
    region: String,
    #[serde(rename = "5. marketOpen")]
    market_open: String,
    #[serde(rename = "6. marketClose")]
    market_close: String,
    #[serde(rename = "7. timezone")]
    timezone: String,
    #[serde(rename = "8. currency")]
    currency: String,
    #[serde(rename = "9. matchScore")]
    match_score: String,
}
