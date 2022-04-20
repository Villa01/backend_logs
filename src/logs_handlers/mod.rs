use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{options::FindOptions, Client};
use std::sync::Mutex;

use bson::{doc};


const MONGO_DB: &'static str = "ProyectoF2";
const MONGO_COLL_LOGS: &'static str = "Logs";

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/get_logs")
            .route(web::get().to(get_logs))
    );
}

async fn get_logs(data: web::Data<Mutex<Client>>) -> impl Responder {
    let logs_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLL_LOGS);

    let find_options = FindOptions::builder().sort(doc! { "_id": -1}).build();
    let mut cursor = logs_collection.find(None, find_options).await.unwrap();

    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
}
