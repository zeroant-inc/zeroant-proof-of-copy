use crate::blockchain::{
    blockchain::BlockChain,
    transaction::{Transaction, TransactionValue},
};
use blockchain::block::Block;
use blockchain::block::Block;
mod blockchain;
fn main() {
    let mut blockchain = BlockChain::new(String::from("zero"));

    let genesis = blockchain.add_genesis(Block::new(
        "0".to_string(),
        Transaction::new(
            "-1".to_string(),
            TransactionValue::Token("first".to_string()),
            "genesis".to_string(),
            "revelation".to_string(),
            0.0,
        ),
        "first".to_string(),
        "genesis".to_string(),
    ));
    let pre = &genesis.digest();
    println!("{}", pre);
    let revelation = Block::new(
        "-1".to_string(),
        Transaction::new(
            "-1".to_string(),
            TransactionValue::Token("first".to_string()),
            "revelation".to_string(),
            "genesis".to_string(),
            0.0,
        ),
        pre.clone(),
        "revelation".to_string(),
    );
    blockchain.add_revelation(revelation);
    let chains = blockchain.get_chain();
    println!("{} {}", chains.len(), blockchain.chains_len());
    let mut _guess: Option<i64> = Some(1);

    while _guess != None {
        let current_block = blockchain.get_current();
        let previous_hash: String = current_block.digest();
        let block = Block::new(
            _guess.expect("").to_string(),
            Transaction::new(
                "-1".to_string(),
                TransactionValue::Token("last".to_string()),
                "14563cessed4raver4".to_string(),
                "dfkjlfelkrgtdklre".to_string(),
                0.0,
            ),
            previous_hash,
            blockchain.get_hostname(),
        );
        blockchain.mine(block);
        let chains = blockchain.get_chain();
        println!("{} {}", &chains.len(), &blockchain.chains_len());
        if _guess.expect("Expected int") == 3 {
            break;
        }
        _guess = Some(_guess.as_ref().expect("") + 1);
    }
    let chains = blockchain.get_chain();
    for block in chains.iterate() {
        println!(
            "block {} {} {}",
            block.get_id(),
            &block.digest(),
            block.get_nonce().expect("nonce expected"),
        )
    }
}
