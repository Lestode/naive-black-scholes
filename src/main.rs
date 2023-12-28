mod data_fetcher;
mod european_options;
#[tokio::main]
async fn main() {
    let df = data_fetcher::new();
    match df.get_current_price("AAPL").await {
        Ok(price) => println!("Current Apple price: {}", price),
        Err(e) => println!("An error occured {}", e),
    }
}
