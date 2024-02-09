use actix_web::{App, HttpServer};
use db::MongoClient;
// use db::MongoClient;
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)] 
    {
        println!("Debug mode. 🪲💣🪦⚰️");
        dotenv().ok();
    }

    let db_client = get_mongodb_client().await.expect("Not possible to create a client to MongoDB!");

    let server_address = env::var("DB_SERVER_ADDRESS").expect("DB_SERVER_ADDRESS is not set!");
    let server_port : u16 = env::var("DB_SERVER_PORT").expect("DB_SERVER_PORT is not set!").parse().expect("DB_SERVER_PORT must be an integer!");

    let server_add_show = match server_address.as_str() {
        "0.0.0.0" => "localhost",
        "127.0.0.1" => "localhost",
        _ => &server_address
    };

    println!("Database API server is running on http://{}:{}", server_add_show, server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(db_client.clone())
            .service(routes::index)
            // .service(routes::chat)
            // .service(web::scope("/app")
            //     .route("/index.html", web::get().to(index)),)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind((server_address, server_port))?
    .run()
    .await
}

async fn get_mongodb_client() -> Option<MongoClient> {
    let mut connection_attemp: u8 = 0;
    loop {
        connection_attemp += 1;

        println!("Attempting create a MongoDB client (Attempt {connection_attemp}/3).");
        
        match db::MongoClient::new().await {
            Ok(client) => {
                println!("MongoDB client created successfully!");
                break Some(client)
            },
            Err(e) => {
                println!("Error creating MongoDB client.");
                println!("{}", e);
                if connection_attemp >= 3 { break None };
                println!("Will try again...")
            },
        }
    }
}