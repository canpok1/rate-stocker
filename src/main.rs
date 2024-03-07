use coincheck_rust::client::Client;

#[tokio::main]
async fn main() {
    let access_key = "";
    let secret_key = "";
    let pairs = ["btc_jpy", "etc_jpy", "mona_jpy", "plt_jpy"];

    let coincheck: coincheck_rust::client::DefaultClient;
    match coincheck_rust::client::DefaultClient::new(&access_key, &secret_key) {
        Ok(cli) => {
            coincheck = cli;
        }
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    for pair in pairs.iter() {
        match coincheck.get_ticker(&pair).await {
            Ok(ticker) => {
                println!("get_ticker({}) => {:?}", pair, ticker);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
