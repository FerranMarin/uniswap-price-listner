use std::collections::HashMap;
use std::str::FromStr;

use web3::Web3;
use web3::contract::{Contract, Options};
use web3::transports::WebSocket;
use web3::types::{Address, FilterBuilder, Blocknumber};

#[derive(Debug)]
struct Pair {
    address: Address,
    decimals: u128,
    symbol: String,
    token0: Address,
    token1: Address,
    price: u128,
    price_block: u128
}


#[tokio::main]
async fn main() -> web3::Result<()> {
    let websocket = web3::transports::WebSocket::new("wss://mainnet.infura.io/ws/v3/b6744db53a18407799db22924e0725db").await?;
    let web3s = web3::Web3::new(websocket);

    let factory_addr = Address::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap();
    let factory_contract = Contract::from_json(web3s.eth(), factory_addr, include_bytes!("factory_abi.json")).unwrap();
    let total_pairs: u128 = factory_contract.query("allPairsLength", (), None, Options::default(), None).await.unwrap();
    println!("Total number of Pairs: {}", total_pairs);
    println!("About to load their information into HashMap");
    let mut uniswap_pairs = HashMap::new();
    for n in 1..=total_pairs {
        println!("Adding pair: {}", n);
        let pair_addr: Address = factory_contract.query("allPairs", n, None, Options::default(), None).await.unwrap();
        let pair_contract = Contract::from_json(web3s.eth(), pair_addr, include_bytes!("pair_abi.json")).unwrap();
        let pair_decimals: u128 = pair_contract.query("decimals", (), None, Options::default(), None).await.unwrap();
        let pair_reserves: (u128, u128, u128) = pair_contract.query("getReserves", (), None, Options::default(), None).await.unwrap();
        let pair_symbol: String = pair_contract.query("symbol", (), None, Options::default(), None).await.unwrap();
        let pair_token0: Address = pair_contract.query("token0", (), None, Options::default(), None).await.unwrap();
        let pair_token1: Address = pair_contract.query("token1", (), None, Options::default(), None).await.unwrap();
        let pair = Pair {
            address: pair_addr,
            decimals: pair_decimals,
            symbol: pair_symbol,
            token0: pair_token0,
            token1: pair_token1,
            price: pair_reserves.0 / pair_reserves.1,
            price_block: pair_reserves.2
        };
        uniswap_pairs.insert(n, pair);
    };
    println!("All pairs done!");

    Ok(())
}


async fn price_listener(pair: Pair, web3s: Web3<WebSocket>) {
    let pair_contract = Contract::from_json(web3s.eth(), pair.address, include_bytes!("pair_abi.json")).unwrap();
    let pair_filter = FilterBuilder::default().address(vec![pair.address]).from_block(pair.price_block).topics(

    ).build()

}