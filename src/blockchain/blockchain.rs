use super::{block::Block, chain::Chain};
pub struct BlockChain {
    hostname: String,
    chains: Chain,
     guess: Option< i64>,
}
impl BlockChain {
    pub fn new(hostname: String) -> Self {
        let chain = Chain::new();
        Self {
            hostname,
            chains: chain,
            guess:Some(999999999999),
        }
    }
}
impl BlockChain {
    pub fn difficulty() -> usize {
        return 2;
    }
    pub fn get_hostname(&self) -> String {
        return self.hostname.clone();
    }
    pub fn get_chain(&mut self) -> &Chain {
        return &self.chains;
    }
    pub fn get_genesis(&mut self) -> &Block {
        return self.chains.first().expect("");
    }
    pub fn chains_len(&self) -> usize {
        return self.chains.len() - 1;
    }
    pub fn get_current(&mut self) -> &Block {
        return self
            .chains
            .get(&self.chains_len() - 1)
            .expect("Out of range");
    }
    pub fn get_revelation(&mut self) -> &Block{
        return self.chains.last().expect("Expected");
    }
    pub fn add_block(&mut self, block: Block) -> &mut Self {
        self.chains.insert(self.chains_len(), block);
        return self;
    }
    pub fn add_genesis(&mut self, block: Block)-> &Block{
        self.chains.add(block);
        return self.chains.first().expect("expected first block");
    }
    pub fn add_revelation(&mut self, block: Block) -> &Block{
        let rev = self.get_revelation_block(block);
        self.chains.insert(self.chains_len()+1, rev);
        return self.chains.last().expect("expected first block");
    }
    pub fn get_revelation_block(&mut self, revelation: Block) -> Block {
        let genesis = self.get_genesis();
        let genesis_hash = genesis.digest();
        return self.find(genesis_hash, revelation);
    }
    fn find(&mut self, genesis_hash: String, revelation: Block) -> Block {
        self.guess = Some(999999999999);
        let hostname = &self.hostname;
        let diff = BlockChain::difficulty();
        let mut revelation = revelation;
        let genesis_hash_last3: String = String::from(&genesis_hash[genesis_hash.len() - diff..]);
        while self.guess != None {
            let count: Option<i64>  =  self.guess.clone() as Option<i64>;
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
                self.guess = None;
              
                return revelation;
            }
            self.guess= Some(count.expect("msg") -1 );
        }

        return revelation;
    }
    pub fn mine(&mut self, _block: Block) -> bool {
        self.guess = Some(999999999999);
        let mut block = _block;
        let genesis = &self.get_genesis();
        let genesis_hash = genesis.digest();
        let revelation = self.get_revelation();
        let revelation_hash = revelation.digest();
        let diff = BlockChain::difficulty();
        let genesis_hash_first3: String = String::from(&genesis_hash[..diff]);
        let revelation_hash_last3: String =
            revelation_hash[revelation_hash.len() - diff..].to_string();
        while  self.guess != None {
            let count: Option<i64>  =  self.guess.clone() as Option<i64>;
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
                self.guess = None;
                self.add_block(block);
                return true;
            }
            self.guess= Some(count.expect("msg") -1 );
        }
        return false;
    }
}
