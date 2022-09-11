use mongodb::Client;

use super::{
    block::Block,
    chain::Chain,
    transaction::{Transaction, TransactionValue},
};
#[derive(Clone)]
pub struct BlockChain {
    db: Client,
    hostname: String,
    chains: Chain,
}

impl BlockChain {
    pub fn new(hostname: String, client: Client) -> Self {
        let chain = Chain::new(
            client.database(&hostname.clone()).collection("blockchain"),
            client.database(&hostname.clone()).collection("record"),
        );
        Self {
            db: client,
            hostname: hostname,
            chains: chain,
        }
    }
}
impl BlockChain {
    pub fn difficulty() -> usize {
        return 2;
    }
    pub fn _get_hostname(&self) -> String {
        return self.hostname.clone();
    }
    pub fn _set_hostname(&self) -> String {
        return self.hostname.clone();
    }
    pub fn get_chain(&self) -> &Chain {
        return &self.chains;
    }
    fn _raw_block<T>(&self) -> mongodb::Collection<T> {
        self.db
            .database(&self.hostname.clone())
            .collection("blockchain")
    }
    pub fn raw_record<T>(&self) -> mongodb::Collection<T> {
        self.db
            .database(&self.hostname.clone())
            .collection("record")
    }
    pub async fn get_genesis(&self) -> Option<Block> {
        let first = self.chains.first().await;
        return match first {
            Ok(ok) => ok,
            Err(_) => None,
        };
    }
    pub async fn chains_len(&self) -> usize {
        return (self.chains.len().await.expect("msg") + 1)
            .try_into()
            .unwrap();
    }
    pub async fn get_previous_hash(&self) -> Option<String> {
        let current = self.get_current().await;
        if current == None {
            let genesis = self.get_genesis().await;
            if genesis == None {
                return None;
            }
            let pre = genesis.expect("").digest();

            return Some(pre);
        }
        let pre = current.expect("").digest();

        return Some(pre);
    }
    pub async fn get_current(&self) -> Option<Block> {
        let current = self.chains.current().await;
        return match current {
            Ok(ok) => ok,
            Err(_) => None,
        };
    }
    pub async fn get_revelation(&self) -> Option<Block> {
        let last = self.chains.last().await;
        return match last {
            Ok(ok) => ok,
            Err(_) => None,
        };
    }
    pub async fn add_block(&self, block: Block) -> &Self {
        self.chains.insert(block).await;
        return self;
    }
    pub async fn add_genesis(&self) -> Option<Block> {
        let block = Block::with_id(
            "0".to_string(),
            Transaction::new(
                "-1".to_string(),
                TransactionValue::token("first".to_string()),
                "genesis".to_string(),
                "revelation".to_string(),
                0.0,
            ),
            "first".to_string(),
            "genesis".to_string(),
        );
        let genesis = match self.get_genesis().await {
            Some(a) => a,
            None => {
                self.raw_record::<Block>().insert_one(block, None).await;
                return Some(self.get_genesis().await).expect("expected first block");
            }
        };
        if block.digest() != genesis.digest() {
            panic!("Invalid genesis block");
        }
        return Some(genesis);
    }
    pub async fn add_revelation(&self) -> Option<Block> {
        let genesis = self.get_genesis().await;
        let pre = genesis.expect("").digest().clone();
        let block = self
            .get_revelation_block(Block::with_id(
                "-1".to_string(),
                Transaction::new(
                    "-1".to_string(),
                    TransactionValue::token("last".to_string()),
                    "revelation".to_string(),
                    "genesis".to_string(),
                    0.0,
                ),
                pre.clone(),
                "revelation".to_string(),
            ))
            .await;
        let revelation = match self.get_revelation().await {
            Some(a) => a,
            None => {
                self.raw_record::<Block>().insert_one(block, None).await;
                return Some(self.get_revelation().await).expect("expected last block");
            }
        };
        if block.digest() != revelation.digest() {
            panic!("Invalid revelation block");
        }
        return Some(revelation);
    }
    pub async fn get_revelation_block(&self, revelation: Block) -> Block {
        let genesis = self.get_genesis().await.expect("msg");
        let genesis_hash = genesis.digest();
        return self.find(genesis_hash, revelation).await;
    }
    async fn find(&self, genesis_hash: String, revelation: Block) -> Block {
        let mut guess = Some(999999999999);
        let hostname = &self.hostname;
        let diff = BlockChain::difficulty();
        let mut revelation = revelation;
        let genesis_hash_last3: String = String::from(&genesis_hash[genesis_hash.len() - diff..]);
        while guess != None {
            let count: Option<i64> = guess.clone() as Option<i64>;
            revelation.set_nonce(count);
            let revelation_hash = revelation.digest();
            let revelation_hash_first3: String = (revelation_hash[..diff]).to_string();
            println!(
                "hash {} hostname {} nonce {}",
                revelation_hash,
                hostname,
                revelation.get_nonce().expect("")
            );
            println!("{} <==> {}", genesis_hash_last3, revelation_hash_first3);
            if revelation_hash_first3 == genesis_hash_last3 {
                return revelation;
            }
            guess = Some(count.expect("msg") - 1);
        }

        return revelation;
    }
    pub async fn mine(&self, block: Block) -> Result<bool, &str> {
        let mut guess = Some(999999999999);
        let mut block = block;
        block.id = Some(self.chains_len().await.to_string());
        let genesis = &self.get_genesis().await.expect("");
        let genesis_hash = genesis.digest();
        let revelation = self.get_revelation().await.expect("");
        let revelation_hash = revelation.digest();
        let diff = BlockChain::difficulty();
        let genesis_hash_first3: String = String::from(&genesis_hash[..diff]);
        let revelation_hash_last3: String =
            revelation_hash[revelation_hash.len() - diff..].to_string();
        let previous_hash = self.get_previous_hash().await;

        if previous_hash == None {
            return Err("Invalid request");
        } else if previous_hash.expect("") != block.previous_hash {
            return Err("Invalid previous_hash");
        }
        while guess != None {
            let count: Option<i64> = guess.clone() as Option<i64>;
            block.set_nonce(count);
            let hash = block.digest();
            println!(
                "hash {} hostname {} nonce {}",
                hash,
                self.hostname,
                block.get_nonce().expect("")
            );
            let block_hash_first3 = String::from(&hash[..diff]);
            let block_hash_last3 = hash[hash.len() - diff..].to_string();
            println!(
                "{} <==> {} {} <==> {}",
                block_hash_first3, genesis_hash_first3, block_hash_last3, revelation_hash_last3
            );
            if block_hash_first3 == genesis_hash_first3 && block_hash_last3 == revelation_hash_last3
            {
                self.add_block(block).await;
                return Ok(true);
            }
            guess = Some(count.expect("msg") - 1);
        }
        return Ok(false);
    }
}
