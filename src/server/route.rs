use std::{
    future::Future,
    iter::{FlatMap, Map},
};

use actix_web::{get, post, web, HttpResponse};
use mongodb::{
    bson::{doc, RawDocument},
    options::IndexOptions,
    Client, Collection, Cursor, IndexModel,
};

use crate::{
    blockchain::{
        self,
        block::Block,
        blockchain::BlockChain,
        transaction::{Transaction, TransactionValue},
    },
    models::user::User,
    server::constant::{COLL_NAME, DB_NAME},
};

/// Adds a new user to the "users" collection in the database.
#[post("/add_user")]
pub async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Gets the user with the supplied username.
#[get("/get_user/{username}")]
pub async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Gets the chain with the supplied username.
#[get("/chains")]
pub async fn get_chains(client: web::Data<BlockChain>) -> HttpResponse {
    let chains = client.get_chain().iterate().await;

    if let Err(err) = chains {
        return HttpResponse::InternalServerError().body(err.to_string());
    }

    match chains {
        Ok(chain) => {
            let mut chain = chain;
            let mut array: Vec<String> = vec![];

            while chain.advance().await.unwrap() {
                // let c = Block {};
                array.push(
                    chain
                        .current()
                        .get_object_id("_id")
                        .expect("")
                        .to_string()
                        .clone(),
                );
                println!(
                    "{:?}",
                    chain.current().get_object_id("_id").expect("").to_string()
                );
            }
            HttpResponse::Ok().json(array)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
// Gets the chain with the supplied username.
#[get("/chains/mine")]
pub async fn chains_mine(blockchain: web::Data<BlockChain>) -> HttpResponse {
    let pre = blockchain
        .get_previous_hash()
        .await
        .expect("expected previous hash");
    let result = blockchain
        .mine(Block::new(
            blockchain.chains_len().await.to_string(),
            Transaction::new(
                "-1".to_string(),
                TransactionValue::token("first".to_string()),
                "genesis".to_string(),
                "revelation".to_string(),
                0.0,
            ),
            pre.clone(),
            "genesis".to_string(),
        ))
        .await;
    match result {
        Ok(_) => return HttpResponse::Ok().json(["Block Added"]),
        Err(error) => return HttpResponse::Ok().json([format!("{}!", error)]),
    };
}
pub async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}
