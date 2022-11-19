use anyhow::Result;
use mongodb::{options::ClientOptions, Client, Collection};

pub async fn init<T>(db: &str, collection: &str) -> Result<MongoRepository<T>> {
    let client = Client::with_options(
        ClientOptions::parse("mongodb://localhost:27017")
            .await
            .expect("MongoDB not reachable"),
    )
    .expect("Could not create client");
    Ok(MongoRepository {
        collection: client.database(db).collection(collection),
    })
}

pub struct MongoRepository<T> {
    pub collection: Collection<T>,
}
