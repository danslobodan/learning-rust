use sha256::digest;

use chrono::Utc;

pub fn example() {
    let mut new_bc = BlockChain::new();
    new_bc.starting_block();

    println!("{:?}", new_bc);

    let new_block = Block::new(2, new_bc.blocks[0].hash.to_owned(), "Slobodan Dan".to_string());
    new_bc.try_add_block(new_block);

    new_bc.is_chain_valid(&new_bc.blocks);
}

#[derive(Debug, Clone)]
struct BlockChain {
    blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    hash: String,
    previous_hash: String,
    timestamp: i64,
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let (nonce, hash) = Block::mine_block(id, timestamp, &previous_hash, &data);

        Block { id, nonce, data, hash, previous_hash, timestamp }
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("Mining block..");
        let mut nonce = 1;
        loop {
            let block_string = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce);
            let hash = digest(block_string);
            if hash.starts_with("0000") {
                println!("Mined! nonce: {}, hash: {}", nonce, hash);
                return (nonce, hash);
            }

            nonce += 1;
        }
    }
}

impl BlockChain {
    fn new() -> Self {
        BlockChain { blocks: vec![] }
    }

    fn starting_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            data: String::from("I am the first or genesis block"),
            previous_hash: String::from(
                "0000000000000000000000000000000000000000000000000000000000000000"
            ),
            nonce: 11316,
            hash: String::from("000015783b764259d382017d91a36d206d0600e2cbb3567748f46a33fe9297cf"),
            timestamp: Utc::now().timestamp(),
        };
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        match self.blocks.last() {
            None => println!("The chain does not have the first block"),
            Some(last_block) => {
                if self.is_block_valid(&block, last_block) {
                    self.blocks.push(block);
                    println!("Block has been successfully added");
                } else {
                    println!("Could not add block, invalid");
                }
            }
        }
    }

    fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.previous_hash != latest_block.hash {
            println!("Block with id {} has wrong previous hash", new_block.id);
            return false;
        } else if !new_block.hash.starts_with("0000") {
            println!("Block with id {} has invalid hash", new_block.id);
            return false;
        } else if new_block.id != latest_block.id + 1 {
            println!(
                "Block with id {} is not the next block after the latest block with id: {}",
                new_block.id,
                latest_block.id
            );
            return false;
        } else if
            digest(
                format!(
                    "{}{}{}{}{}",
                    new_block.id,
                    &new_block.previous_hash,
                    &new_block.data,
                    new_block.timestamp,
                    new_block.nonce
                )
            ) != new_block.hash
        {
            println!("Block with id {} has invalid hash", new_block.id);
            return false;
        }

        true
    }

    fn is_chain_valid(&self, chain: &Vec<Block>) -> bool {
        match chain.len() {
            0 => println!("The chain is empty"),
            1 => println!("The chain contains a single block"),
            _ => {
                for i in 1..chain.len() {
                    let previous = chain.get(i - 1).unwrap();
                    let current = chain.get(i).unwrap();

                    if !self.is_block_valid(current, previous) {
                        return false;
                    }
                }
            }
        }

        println!("The chain is valid");
        true
    }
}
