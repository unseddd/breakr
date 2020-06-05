extern crate config;
extern crate hyper;

#[macro_use]
extern crate failure;

use std::{collections::HashMap, fs::File, io::Write};

use clap::{Arg, App};

use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Method, Request};
use hyper::body::HttpBody;

use tokio;

async fn download_contract(address: &str) -> Result<(), failure::Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let config = load_config()?;

    let req_uri = format!("https://api.etherscan.io/api?module=contract&action=getabi&address={}&apikey={}", address, config["api_key"]);

    let req = Request::builder().method(Method::GET).uri(req_uri).body(Body::default())?;
    let mut res = client.request(req).await?;

    // get response body, and convert ABI bytes to a hex-string
    let contract = if let Some(contract_res) = res.body_mut().data().await {
        contract_res?
    } else {
        return Err(format_err!("failed to extract contract response"));
    };

    let contract_str = contract.iter().map(|b| format!("{:02x}", b) ).collect::<Vec<String>>().join("");
    let contract_bytes = contract_str.as_bytes();

    // write contract bytes to a file
    let mut contract_file = File::create(format!("fuzz/contracts/{}.bin", address))?;
    contract_file.write_all(&contract_bytes)?;

    // write contract bytes to the file for fuzzing
    let mut fuzz_file = File::create("fuzz/contracts/fuzz.bin")?;
    fuzz_file.write_all(&contract_bytes)?;

    Ok(())
}

fn load_config() -> Result<HashMap<String, String>, failure::Error> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings"))?
        .merge(config::Environment::with_prefix("APP"))?;

    settings.try_into::<HashMap<String, String>>().map_err(|e| e.into())
}


#[tokio::main]
async fn main() {
    let matches = App::new("breakr")
        .version("0.0.1")
        .author("unseddd")
        .arg(Arg::with_name("contract")
             .short("c")
             .long("contract")
             .takes_value(true)
             .required(true)
             .help("Address of the contract to fetch, format: 0x<40-hex-chars>"))
        .get_matches();

    if let Some(address) = matches.value_of("contract") {
        if let Err(e) = download_contract(address).await {
            panic!("Error downloading contract: {:?}", e);
        }
    }

    std::process::Command::new("cargo")
        .arg("fuzz")
        .arg("run")
        .arg("evm")
        .spawn()
        .expect("failed to run EVM fuzzer");
}
