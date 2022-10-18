use crate::models::{Block, Blockchain};
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Mutex;
#[get("/mine_block")]
pub async fn mine_block(chain: web::Data<Mutex<Blockchain>>) -> impl Responder {
    // get the chain in AppState
    // get the last block details
    let mut blockchain = chain.lock().unwrap();
    let pre_block = blockchain.chain.last().unwrap().to_owned();
    let pre_block_index = pre_block.index;
    let pre_block_proof = pre_block.proof;
    let pre_block_hash = pre_block.clone().calculate_hash();
    // create new block
    let proof = Blockchain::proof_of_work(blockchain.clone(), pre_block_proof);
    let new_block = Block::new(&pre_block_index + 1, proof, pre_block_hash);
    blockchain.chain.push(new_block.clone());
    // add the block to the chain
    HttpResponse::Ok().body(format!("{}th block mined", &pre_block_index+1))
}

#[get("/get_chain")]
pub async fn get_chain(chain: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let blockchain = chain.lock().unwrap();
    let chain = blockchain.chain.iter().map(|chain|{serde_json::to_string(&chain).unwrap()}).collect::<String>();
    HttpResponse::Ok().body(chain)
}
