use actix_web::{web, App, HttpServer};
use bootstrap::bootstrap;
use std::env;

mod bootstrap;
pub mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = bootstrap().await;

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(handler::account::create_account)
            .service(handler::account::get_account_by_id)
            .service(handler::transaction::create_deposit)
            .service(handler::transaction::create_withdrawal)
            .service(handler::transaction::create_transfer)
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await
}
