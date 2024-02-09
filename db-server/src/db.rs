use mongodb::{error, options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
pub struct MongoClient {pub client: Client}

impl MongoClient {
    pub async fn new() -> Result<MongoClient, actix_web::Error> {
        let client = create_mongo_client().await
            .map_err(|_| {
                actix_web::error::ErrorInternalServerError("Invalid MongoDB credentials")
            })
            .unwrap();

        let db_list = client.list_database_names(mongodb::bson::doc! {}, None).await;
        // // let transaction_options = mongodb::options::TransactionOptions::builder().build();
        // let options = mongodb::options::SessionOptions::builder().build();
        //     // .default_transaction_options(transaction_options)
        // // let session = client.start_session(options).await?;
        match db_list {
            Ok(_) => Ok(MongoClient{ client }),
            Err(_) => Err(actix_web::error::ErrorInternalServerError("Invalid MongoDB credentials")),
        }
        
    }

    pub async fn insert_document<T: Serialize>(
        &self,  
        db_name: &str, 
        collection: &str, 
        data: T, 
    ) -> error::Result<mongodb::results::InsertOneResult> {
        let coll : Collection<T> = self.client.database(db_name).collection::<T>(collection);

        coll.insert_one(data, None).await
    }
}

struct MongoCredentials {
    user: String,
    pwd: String,
    address: String,
    port: String,
}

impl MongoCredentials {
    fn new() -> MongoCredentials {
        let user = env::var("MONGODB_USER").expect("Database user is not defined!");
        let pwd = env::var("MONGODB_PASS").expect("Database password is not defined!");
        let address = env::var("MONGODB_ADDR").expect("Database server address is not defined!");
        let port = env::var("MONGODB_PORT").expect("Database server port is not defined!");

        MongoCredentials { user, pwd, address, port }
    }
}

fn get_mongo_uri() -> String {
    let credentials = MongoCredentials::new();
    format!("mongodb://{}:{}@{}:{}", credentials.user, credentials.pwd, credentials.address, credentials.port)
}

async fn create_mongo_client() -> error::Result<Client> {

    let mongo_uri = get_mongo_uri();
    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;

    Ok(client)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub db_name: String,
    pub coll_name: String,
}