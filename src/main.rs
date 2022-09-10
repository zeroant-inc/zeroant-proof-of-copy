use crate::blockchain::blockchain::BlockChain;
use mongodb::Client;
use server::{route::create_username_index, Server};
mod blockchain;
mod models;
mod server;
mod wallet;

#[actix_web::main]
async fn main() {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;
    let blockchain = BlockChain::new(String::from("zero"), client.clone());
    blockchain.add_genesis().await;
    blockchain.add_revelation().await;
    if let Err(error) = Server::listen(client, blockchain).await {
        println!("error {}", error);
    }
}
