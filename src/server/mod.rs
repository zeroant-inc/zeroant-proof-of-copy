pub mod constant;
pub mod route;
use crate::blockchain::blockchain::BlockChain;
use actix_web::{web, App, HttpServer};
use mongodb::Client;

pub struct Server {}

impl Server {
    /// Creates an index on the "username" field to force the values to be unique.
    pub async fn listen(client: Client, blockchain: BlockChain) -> Result<(), std::io::Error> {
        let addr = ("0.0.0.0", 8080);
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(client.clone()))
                .app_data(web::Data::new(blockchain.clone()))
                .service(route::add_user)
                .service(route::get_user)
                .service(route::get_chains)
                .service(route::chains_mine)
        })
        .bind(addr)?
        .run();
        println!("server running htt://{}:{}", addr.0, addr.1);
        return server.await;
    }
}
