mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, App, HttpServer};
use api::expenditure_routes::{create, delete, get_all, get_one, test_create, update};
use repository::{expenditure_document::ExpenditureDocument, mongo_repository::init};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let db = "myDB";
        let collection_name = "expenditure";
        App::new()
            .wrap(logger)
            .data_factory(|| async {
                let init = init::<ExpenditureDocument>(db, collection_name).await;
                match init {
                    Ok(x) => {
                        println!("{}", x.collection.name());
                        Ok(x)
                    }
                    Err(e) => {
                        println!("{}", e);
                        Err(e)
                    }
                }
            })
            .service(create)
            .service(get_all)
            .service(get_one)
            .service(update)
            .service(delete)
            .service(test_create)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
