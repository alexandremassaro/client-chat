use actix_web::{web, get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db::{self, MongoClient};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Database API server.")
}

#[post("/chat")]
async fn chat(message_received: web::Query<MessageReceived>) -> impl Responder {
    println!("{:?}", message_received);
    HttpResponse::Ok().body(format!("
        Chat room : {}\n
        - {}: {}
        ", 
        message_received.conversation_id,
        message_received.sender_id,
        message_received.message,
    ))
}

#[post("/insert/{db}/{collection}")]
async fn insert<T: Serialize + for<'a> Deserialize<'a>>(db_client: web::Data<MongoClient>, coll_info: web::Query<db::CollectionInfo>, data: web::Json<T>) -> impl Responder {
    let insert_result = db_client.insert_document(&coll_info.db_name, &coll_info.coll_name, data.into()).await;
    

    HttpResponse::Ok().body(format!("
        Chat room : {}\n
        - {}: {}
        ", 
        message_received.conversation_id,
        message_received.sender_id,
        message_received.message,
    ))
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }