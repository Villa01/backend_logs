use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::*;

mod logs_handlers;
mod time_handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    let mut client_options = ClientOptions::parse("mongodb://root:password@35.202.76.252:27017/").await.unwrap();
 
    client_options.app_name = Some("Proyecto2".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new().app_data(client.clone()).service(
            web::scope("/api")
                .configure(logs_handlers::scoped_config)
                .configure(time_handlers::scoped_config),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
