// here we create the blockchian with the helpo basic data structures

use chrono::{Local, Utc};
use sha256::digest;

#[derive(Debug, Clone)]
struct Blockchain {
    block: Vec<Block>,
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

// first create the empty block in the block

impl Blockchain {
    // creating the empty block , which is return the vector of block
    fn new() -> Self {
        Self { block: vec![] }
    }

    // now create the genesis block -> first block in the blokchain
    // we have hard coded it. from the reference of this block we can generate the other blocks
    fn startin_block(&mut self) {
        // inside this block we have a block is called genesis block, which have information for creating the first block in the blockhain
        let genesis_block = Block {
            id: 1,
            data: String::from("I am first or genesis block in the blockhain"),
            previous_hash: String::from(
                "0000000000000000000000000000000000000000000000000000000000000000",
            ),
            nonce: 1139,
            hash: String::from("00006d1fd6e400dc0de0b5d4a051146a6e83f8949f5ade42b1236d39bc24bf53"),
            timestamp: Utc::now().timestamp(),
        };
        // finally add the first block in the blockchain using the push function. pushing this block where we have first created the empty block
        self.block.push(genesis_block);
    }

    // this function is used to add one more block

    fn try_add_block(&mut self, block: Block) {
        // here we check there any block in the blockchain
        // if the hrere was not any block we canot the block in the blockchain
        //(there must be first or genesis block already in the block)
        match self.block.last() {
            None => {
                println!("The blockchain does not have already one block");
                return;
            }
            Some(latest_block) => {
                // there three rules of mines
                // 1) its previous hash needs to actually match the hash of the last block in the chain
                // 2) the hash of the block needs to start with the four zeros, which means that it has signed and it has been mined correcltu
                // 3) the idea of block that we intend to add needs to needs to be the last ID of the block in the chian incremeneted by one
                // 4) the fourth condition is will be that the hash needs to actually be correct(the filed of the block when concatenated together needs to give us block hash)
                // Again, we will take care of all these conditions inside a separate function called is block vaild

                // adding the is_block_valid or not

                if self.is_block_valid(&block, latest_block) {
                    self.block.push(block);
                    println!("block is successfully added");
                } else {
                    println!("Block could not be added, Invalid!");
                }
            }
        }
    }

    fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        // Check if the previous hash matches
        if new_block.previous_hash != latest_block.hash {
            println!("Block with id {} has wrong previous hash", new_block.id);
            return false;
        }

        // Check if the block's hash starts with "0000"
        if !new_block.hash.starts_with("0000") {
            println!("Block with id {} has invalid hash", new_block.id);
            return false;
        }

        // Check if the block id is the next block (latest_block.id + 1)
        if new_block.id != latest_block.id + 1 {
            println!(
                "Block with id {} is not the next block after the latest block with id: {}",
                new_block.id, latest_block.id
            );
            return false;
        }

        // Check if the block's hash matches the hash generated from its fields
        let calculated_hash = digest(format!(
            "{}{}{}{}{}",
            new_block.id,
            new_block.previous_hash,
            new_block.data,
            new_block.timestamp,
            new_block.nonce
        ));
        if calculated_hash != new_block.hash {
            println!("Block with id {} has an invalid hash", new_block.id);
            return false;
        }

        // If all checks pass, return true
        true
    }

    // Now we are going to check the block is valid or not.  in the code we are comparig the current and previous block . if the current and previous block is match then we are return the ture, othewise false

    fn is_chain_valid(&self, chain: &Vec<Block>) -> bool {
        match chain.len() {
            0 => println!("chain is empty"),
            1 => println!("chain only contains a single block"),
            _ => {
                for i in 1..chain.len() {
                    // get the previous block and current block then compare it
                    let previous = chain.get(i - 1).unwrap();
                    let current = chain.get(i).unwrap();

                    if !self.is_block_valid(current, previous) {
                        return false;
                    }
                }
            }
        }
        println!("The chain is found to be correct and valid");
        true
    }

    // In the below function we are going the check copy of the chain. for the example we are updating the chain of blocks according to the previous block. some times we found the chain is greater length than our chain in that case we keep our block
    // this function is only used when the blocchain is distributed in nature
    // currently we are considering the chain which is residing in the local/ single machine
    fn chain_selector(&self, local: &Vec<Block>, remote: &Vec<Block>) -> Option<Vec<Block>> {
        let is_valid_local = self.is_chain_valid(&local);
        let is_valid_remote = self.is_chain_valid(&remote);
        match (is_valid_local, is_valid_remote) {
            (true, true) => {
                if local.len() >= remote.len() {
                    println!("The local copy is vaild");
                    Some(local.clone())
                } else {
                    println!("The remote copy is is vaild");
                    Some(remote.clone())
                }
            }
            (true, false) => {
                println!("The local copy is valid, returning the local");
                Some(local.clone())
            }
            (false, true) => {
                println!("The local copy is invalid, returning the remote");
                Some(remote.clone())
            }
            (false, false) => {
                println!("Both the local and remote copy are invalid, returning nothing");
                None
            }
        }
    }
}

// now implement the block with the reference of genesis block
impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        // add the utc by manually
        let now = Utc::now();
        let now_timestamp = now.timestamp();

        // now store the result after the mine_block
        let (nonce, hash) = Block::mine_block(id, now_timestamp, &previous_hash, &data);

        // finally return the self of block which has the previous block value
        Self {
            id,
            nonce,
            data,
            hash,
            previous_hash,
            timestamp: now_timestamp,
        }
    }
    // let define the mine / the input of this block is comes from the all fields of the block
    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("mine_block ..............");
        let mut nonce = 1;
        // now we want to see the in hash having the four zeros in starting , for that we are using the loop to compute the
        // nonce is computed and compute the starting the four's zeros
        loop {
            // declare a variable which is concatenated with the all the fields of the block of the string
            let block_String = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce);
            // now going to calculate the hash of the block of string by using sha256
            // digest is used the compute the hash for the gievn string
            let hash = digest(block_String);

            // now check the nonce which have the fours starting or not.
            // if the it have four zeros at the starting then we can tell the user that the block has been mined
            if hash.starts_with("0000") {
                println!("mined! nonce: {}, hash: {}", nonce, hash);
                return (nonce, hash);
            }
            // we will increment the nounce and compute the next nonce
            nonce += 1;
        }
    }
}

fn main() {
    // now call the blocchain in the main function
    let mut new_BC = Blockchain::new();
    new_BC.startin_block();
    // lets display the blockchain
    println!("{:?}", new_BC);

    let new_block = Block::new(2, new_BC.block[0].hash.to_owned(), "Bhupal".to_string());
    // println!("{:?}", new_block);

    // // add the block to blockchain
    // new_BC.block.push(new_block);
    // println!("{:?}", new_BC);
    // now add the block to blockchain
    new_BC.try_add_block(new_block);

    new_BC.is_chain_valid(&new_BC.block);

    // trying to add more block on it

    let new_block = Block::new(
        3,
        new_BC.block[1].hash.to_owned(),
        "Addinf the 2nd block in the block chan".to_string(),
    );
    new_BC.try_add_block(new_block);

    new_BC.is_chain_valid(&new_BC.block);

    let new_block = Block::new(
        4,
        new_BC.block[2].hash.to_owned(),
        "Adding the 3rd block in the block chain".to_string(),
    );
    new_BC.try_add_block(new_block);

    // checking thr block is valid or not that was we have created in the above
    new_BC.is_chain_valid(&new_BC.block);

    // using the chain_selector function here
    new_BC.block = new_BC
        .chain_selector(&new_BC.block, &new_BC.block)
        .expect("Failed to select a valid chain");

    // i have only single chain in this program. so that i will used it twicly
}
