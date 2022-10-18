use super::block::Block;
use sha256::digest;

#[derive(Clone, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}
impl Blockchain {
    /// this fn is for the first time use to create a chain
    pub fn new_chain(g_block: Block) -> Self {
        let mut chain: Vec<Block> = Vec::new();
        chain.push(g_block);
        let blockchain = Blockchain { chain: chain };
        blockchain
    }
    /// this fn is to add a new block to the chain
    pub fn add_block(mut self, new_block: Block) -> Self {
        self.chain.push(new_block);
        self
    }
    /// this fn is to get the last block of the chain
    pub fn get_prev_block(self) -> Block {
        let prev_block = self.chain.last().unwrap();
        prev_block.clone()
    }

    /// proof of work fn is to generate the proof/nonce and  
    /// aslo validate the proof
    pub fn proof_of_work(self, pre_proof: u64) -> u64 {
        let mut nonce: u64 = 1; // set initial value to 1 cause it's starting from 1
        let mut check_proof = false; //bool value to check the proof and make it false initially
                                     // increase nonce in a loop and check

        while check_proof == false {
            if nonce >= pre_proof {
                let data_to_hash = (nonce.pow(2) - pre_proof.pow(2)).to_string(); //find the nonce
                let hash_operation = digest(data_to_hash); // try to make the hash with the nonce
                                                           // validate the hash is acceptable or not
                                                           // for that get the first 4 char, check those are "0" or not
                let fst_4_char: String = hash_operation.clone().chars().take(4).collect();
                if fst_4_char.trim() == "0000" {
                    check_proof = true;
                } else {
                    nonce += 1;
                }
            } else {
                nonce += 1;
            }
        }
        return nonce;
    }
}
