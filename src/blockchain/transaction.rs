use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum TransactionValue {
    token(String),
    bit(f64),
}

impl TransactionValue {
    fn value_bit(&self) -> f64 {
        match self {
            TransactionValue::bit(val) => return val.clone(),
            TransactionValue::token(_) => panic!(),
        }
    }
    fn value_string(&self) -> String {
        match self {
            TransactionValue::bit(_) => panic!(),
            TransactionValue::token(val) => return val.clone(),
        }
    }
    fn string(&self) -> String {
        match self {
            TransactionValue::bit(val) => return val.to_string(),
            TransactionValue::token(val) => return val.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    pub id: String,
    pub value: TransactionValue,
    pub sender: String,
    pub receiver: String,
    pub fees: f64,
}

impl Transaction {
    pub fn new(
        id: String,
        value: TransactionValue,
        sender: String,
        receiver: String,
        fees: f64,
    ) -> Transaction {
        Transaction {
            id: id,
            value,
            sender: sender,
            receiver: receiver,
            fees: fees,
        }
    }
}
impl Transaction {
    pub fn string(&self) -> String {
        let mut result = String::from("");
        result.push_str(&self.id);
        result.push_str(&self.value.string());
        result.push_str(&self.sender);
        result.push_str(&self.receiver);
        result.push_str(&self.fees.to_string());
        return result;
    }
    pub fn get_id(&self) -> &String {
        return &self.id;
    }
}
