use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{FixedOffset, Utc};
use clap::Parser;
use coincheck_rust::client::Client;

use fetcher::{
    create_rate_type, establish_connection, models::NewDailyRate, select_rate_type,
    upsert_daily_rate,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets coin pair (ex)btc_jpy,etc_jpy,lsk_jpy,mona_jpy,plt_jpy,fnct_jpy,dai_jpy,wbtc_jpy
    pair: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let pairs: Vec<&str> = cli.pair.split(',').collect();
    println!("pair:{:?}", pairs);

    let access_key = "";
    let secret_key = "";

    let coincheck = match coincheck_rust::client::DefaultClient::new(access_key, secret_key) {
        Ok(cli) => cli,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let conn = &mut establish_connection();

    for pair in pairs.iter() {
        let rate_type = match select_rate_type(conn, pair) {
            Some(v) => v,
            None => create_rate_type(conn, pair),
        };

        match coincheck.get_ticker(pair).await {
            Ok(ticker) => {
                let utc_now_datetime = Utc::now();
                let jst_now_datetime =
                    utc_now_datetime.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
                let jst_now_date = jst_now_datetime.date_naive();

                let rate = NewDailyRate {
                    rate_type_id: &rate_type.id,
                    rate_date: &jst_now_date,
                    closing_rate: BigDecimal::from_f64(ticker.last).unwrap(),
                    opening_rate: BigDecimal::from_f64(ticker.last).unwrap(),
                    high_rate: BigDecimal::from_f64(ticker.high).unwrap(),
                    low_rate: BigDecimal::from_f64(ticker.low).unwrap(),
                    volume: BigDecimal::from_f64(ticker.volume).unwrap(),
                };
                upsert_daily_rate(conn, &rate);
                println!("get_ticker({}) => {:?}", pair, ticker);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
