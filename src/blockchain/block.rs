use sha256;
use super::transaction::{Transaction};

#[derive( Clone)]
pub  struct Block  {
    pub id:String,
    pub  transaction: Transaction ,
    pub  previous_hash:String,
    pub owner: String,
   pub nonce:  Option<i64>,
}

impl  Block {
    pub fn new(id: String, transaction: Transaction, previous_hash: String, owner: String) -> Self  {
        Self  {
            id:String::from(id),
            transaction:transaction,
            previous_hash:String::from(previous_hash),
            owner:String::from(owner),
            nonce: Some(999999999999),
        }
    }
}
impl Block {
    pub fn set_nonce(&mut self, nonce:Option<i64>)  {
        self.nonce = nonce;
    }
    
    pub fn get_nonce(&self) -> Option<&i64>{
        return self.nonce.as_ref();
    }
    pub fn string(&self) -> String {
        let mut result = String::from("");
        result.push_str(&self.id);
        result.push_str(&self.transaction.string());
        result.push_str(&self.previous_hash);
        result.push_str(&self.owner);
        if self.nonce.is_none() {
            result.push_str("");
        } else {
            result.push_str(&self.nonce.as_ref().expect("").to_string());
        }
        return result;
    }
    pub fn digest(&self) -> String {
        let a:& String = &self.string();
        let b: String = sha256::digest(a);
        return b;
    }
    pub fn get_id(&self) -> &String{
        return &self.id;
    }
    
}

