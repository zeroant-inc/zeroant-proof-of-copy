use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{FindOneOptions, UpdateModifications},
    results::{InsertOneResult, UpdateResult},
    Collection,
};

use super::block::Block;
#[derive(Clone)]
pub struct Chain {
    collection: Collection<Block>,
    record: Collection<Block>,
}
impl Chain {
    pub fn new(client: Collection<Block>, record: Collection<Block>) -> Self {
        return Self {
            collection: client,
            record: record,
        };
    }
    pub async fn add(&self, block: Block) -> Result<InsertOneResult, mongodb::error::Error> {
        let collection: Collection<Block> = self.collection.clone();
        return collection.insert_one(block, None).await;
    }
    pub async fn insert(&self, block: Block) -> Result<InsertOneResult, mongodb::error::Error> {
        let collection: Collection<Block> = self.collection.clone();
        return collection.insert_one(block, None).await;
    }
    pub async fn len(&self) -> Result<u64, mongodb::error::Error> {
        let collection: Collection<Block> = self.collection.clone();
        collection.count_documents(None, None).await
    }
    pub async fn first(&self) -> Result<std::option::Option<Block>, mongodb::error::Error> {
        let record: Collection<Block> = self.record.clone();
        let query = doc! {
            "owner"  :  "genesis",
        };
        let first = record.find_one(query, None).await;
        return match first {
            Ok(Some(genesis)) => Ok(Some(genesis)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        };
    }
    pub async fn current(&self) -> Result<std::option::Option<Block>, mongodb::error::Error> {
        let collection: Collection<Block> = self.collection.clone();
        let query = doc! {};
        let _o = FindOneOptions::builder()
            .sort(doc! {
                "_id":-1,
            })
            .build();
        let options = Some(_o);

        let current = collection.find_one(query, options).await;
        return match current {
            Ok(Some(current)) => Ok(Some(current)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        };
    }
    pub async fn last(&self) -> Result<std::option::Option<Block>, mongodb::error::Error> {
        let record: Collection<Block> = self.record.clone();
        let query = doc! {
            "owner"  :  "revelation",
        };
        let last = record.find_one(query, None).await;
        return match last {
            Ok(Some(revelation)) => Ok(Some(revelation)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        };
    }
    pub async fn get(
        &self,
        id: String,
    ) -> Result<std::option::Option<Block>, mongodb::error::Error> {
        let collection: Collection<Block> = self.collection.clone();
        let query = doc! {
            "id":id,
        };
        collection.find_one(query, None).await
    }
    pub async fn iterate(&self) -> Result<mongodb::Cursor<Block>, mongodb::error::Error> {
        let collection: Collection<Block> = self.collection.clone();
        let query = doc! {};
        collection.find(query, None).await
    }
}
