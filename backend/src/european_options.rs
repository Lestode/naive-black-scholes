use statrs::distribution::{ContinuousCDF, Normal};
use std::f64::consts::E;

pub trait Option {
    fn price(&self) -> Result<f64, String>;
}
pub struct EuropeanCall {
    current_price: f64,
    strike_price: f64,
    time_to_maturity: f64,
    risk_free_rate: f64,
    volatility: f64,
}

impl Option for EuropeanCall {
    fn price(&self) -> Result<f64, String> {
        let d1 = d1(
            self.current_price,
            self.strike_price,
            self.time_to_maturity,
            self.risk_free_rate,
            self.volatility,
        );
        let d2 = d2(d1, self.volatility, self.time_to_maturity);

        let normal_dist = match Normal::new(0.0, 1.0) {
            Ok(dist) => dist,
            Err(e) => return Err(format!("cannot create normal distribution: {}", e)),
        };

        Ok(self.current_price * normal_dist.cdf(d1)
            - self.strike_price
                * E.powf(-self.risk_free_rate * self.time_to_maturity)
                * normal_dist.cdf(d2))
    }
}

pub struct EuropeanPut {
    current_price: f64,
    strike_price: f64,
    time_to_maturity: f64,
    risk_free_rate: f64,
    volatility: f64,
}

pub fn new_european_put(
    current_price: f64,
    strike_price: f64,
    time_to_maturity: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> EuropeanPut {
    EuropeanPut {
        current_price,
        strike_price,
        time_to_maturity,
        risk_free_rate,
        volatility,
    }
}

impl Option for EuropeanPut {
    fn price(&self) -> Result<f64, String> {
        let d1 = d1(
            self.current_price,
            self.strike_price,
            self.time_to_maturity,
            self.risk_free_rate,
            self.volatility,
        );
        let d2 = d2(d1, self.volatility, self.time_to_maturity);

        let normal_dist = match Normal::new(0.0, 1.0) {
            Ok(dist) => dist,
            Err(e) => return Err(format!("cannot create normal distribution: {}", e)),
        };

        Ok(self.strike_price
            * E.powf(-self.risk_free_rate * self.time_to_maturity)
            * normal_dist.cdf(-d2)
            - self.current_price * normal_dist.cdf(-d1))
    }
}

fn d1(
    current_price: f64,
    strike_price: f64,
    time_to_maturity: f64,
    risk_free_rate: f64,
    volatility: f64,
) -> f64 {
    let numerator = (current_price / strike_price).ln()
        + (risk_free_rate + (volatility.powi(2)) / 2.0) * time_to_maturity;
    let denominator = volatility * time_to_maturity.sqrt();

    numerator / denominator
}

fn d2(d1: f64, volatility: f64, time_to_maturity: f64) -> f64 {
    d1 - volatility * time_to_maturity.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn truncate_f64(value: f64, decimals: usize) -> f64 {
        let formatter = format!("{:.*}", decimals, value);
        formatter.parse::<f64>().unwrap_or(0.0)
    }

    #[test]
    fn test_european_put_price() {
        let european_put = EuropeanPut {
            current_price: 52.0,
            strike_price: 50.0,
            time_to_maturity: 0.5,
            risk_free_rate: 0.05,
            volatility: 0.12,
        };

        let expected_price = 0.554;
        let actual_price = truncate_f64(european_put.price().unwrap(), 3);

        assert_eq!(actual_price, expected_price);
    }
    //test data obtained from https://navi.com/blog/black-scholes-model/
    #[test]
    fn test_european_call_price() {
        let european_call = EuropeanCall {
            current_price: 52.0,
            strike_price: 50.0,
            time_to_maturity: 0.5,
            risk_free_rate: 0.05,
            volatility: 0.12,
        };

        let expected_price = 3.788;
        let actual_price = truncate_f64(european_call.price().unwrap(), 3);

        assert_eq!(actual_price, expected_price);
    }
}
