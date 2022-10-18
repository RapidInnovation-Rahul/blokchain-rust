use actix_web::{services, web::Data, App, HttpServer};
use mylib::*;
use std::sync::Mutex;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // create a genesis block
    let g_block = Block::new(1, 1, "0".to_string());
    let chain = Blockchain::new_chain(g_block);
    let data = Data::new(Mutex::new(chain.clone()));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(services![mine_block, get_chain])
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
