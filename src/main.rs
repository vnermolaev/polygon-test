use std::sync::Arc;
use std::time::Duration;
use ethers::prelude::{Address, Http, Provider, StreamExt};

// Include generated content.
include!(concat!(env!("OUT_DIR"), "/test_contract.rs"));

const ENDPOINT: &str = "https://damp-attentive-theorem.matic-testnet.discover.quiknode.pro/174bc0610f3b5b7bf1c14253684d01e4c20365b3/";

const ADDRESS: &str = "0xBE271e9ef89eaF48c4EcF795b9A2861A8f7EfdC5";

/// A transaction triggering an event was included in the block 37626783.
/// The blockchain is going to be explored starting START_BLOCK.
const START_BLOCK: u64 = 37626780;

#[tokio::main]
async fn main()-> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from(ENDPOINT)?
        .interval(Duration::from_secs(10));

    let contract_address = ADDRESS.parse::<Address>()?;
    let contract = TestContract::new(contract_address, Arc::new(provider));

    let nonce = contract.get().await?;
    println!("nonce: {nonce:?}");

    let events = contract.event::<NonceEventFilter>().from_block(START_BLOCK);
    let mut stream = events.stream().await?;
    let event =  stream.next().await;
    println!("{event:?}");

    Ok(())
}
