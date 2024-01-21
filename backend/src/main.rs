use clap::{command, value_parser, Arg, Command};
use data_fetcher::DataFetcher;
use european_options::EuropeanCall;
use european_options::Option;
use statrs::statistics::Data;
use std::error::Error;
use std::{iter::Successors, sync::Arc};

mod data_fetcher;
mod european_options;
#[tokio::main]
async fn main() {
    let matches = Command::new("black scholes rust")
        .version("0.0.0")
        .author("Louis Barinka")
        .about("naive implementation of black scholes and implied volatility calculation")
        .subcommand(
            Command::new("get_last_price")
                .about("get the last price of the passed symbol")
                .arg(Arg::new("symbol").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("get_last_price", sub_matches)) => {
            let symbol = sub_matches.value_of("symbol").expect("symbol is required");
            command_get_last_price(symbol).await;
        }
        _ => unreachable!("Should prevent reaching this state"),
    }
}

async fn command_get_last_price(symbol: &str) {
    let df = data_fetcher::new();
    match df.get_last_price(symbol).await {
        Ok(price) => println!("Current Apple price: {}", price),
        Err(e) => println!("An error occured {}", e),
    }
}

async fn price_european_put(
    symbol: &str,
    strike_price: f64,
    time_to_maturity: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> Result<(), Box<dyn Error>> {
    let current_price = data_fetcher::new().get_last_price(symbol).await?;
    let european_put = european_options::new_european_put(
        current_price,
        strike_price,
        time_to_maturity,
        risk_free_rate,
        volatility,
    );
    println!("The price of the put would be: {}", european_put.price()?);
    Ok(())
}
