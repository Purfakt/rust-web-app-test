use actix_web::{
    delete, get, patch, post,
    web::Json,
    web::{Data, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::expenditure_dto::{ExpenditureCreate, ExpenditureRead},
    model::expenditure::User,
    repository::expenditure_document::ExpenditureDocument,
};
use crate::{api::utils::result_to_response, model::expenditure::Cost};
use crate::{model::expenditure::Category, repository::mongo_repository::MongoRepository};

#[derive(Deserialize, Serialize)]
pub struct ExpenditureIdentifier {
    expenditure_id: String,
}

#[get("/expenditure/{expenditure_id}")]
pub async fn get_one(
    db: Data<MongoRepository<ExpenditureDocument>>,
    task_id: Path<ExpenditureIdentifier>,
) -> HttpResponse {
    let id = Uuid::try_parse(task_id.into_inner().expenditure_id.as_str()).unwrap();
    result_to_response(db.get_one(&id).await, ExpenditureRead::from_model)
}

#[get("/expenditure")]
pub async fn get_all(db: Data<MongoRepository<ExpenditureDocument>>) -> HttpResponse {
    result_to_response(db.get_all().await, ExpenditureRead::from_model)
}

#[post("/expenditure")]
pub async fn create(
    db: Data<MongoRepository<ExpenditureDocument>>,
    create_dto: Json<ExpenditureCreate>,
) -> HttpResponse {
    let expenditure = create_dto.into_inner().into_model(uuid::Uuid::new_v4());
    result_to_response(db.create(&expenditure).await, ExpenditureRead::from_model)
}

#[patch("/expenditure/{expenditure_id}")]
pub async fn update(
    db: Data<MongoRepository<ExpenditureDocument>>,
    task_id: Path<ExpenditureIdentifier>,
    update_dto: Json<ExpenditureCreate>,
) -> HttpResponse {
    let id = Uuid::parse_str(task_id.into_inner().expenditure_id.as_str()).unwrap();
    let expenditure = update_dto.into_inner().into_model(id);
    result_to_response(
        db.update(&id, &expenditure).await,
        ExpenditureRead::from_model,
    )
}

#[delete("/expenditure/{expenditure_id}")]
pub async fn delete(
    db: Data<MongoRepository<ExpenditureDocument>>,
    task_id: Path<ExpenditureIdentifier>,
) -> HttpResponse {
    let id = Uuid::parse_str(task_id.into_inner().expenditure_id.as_str()).unwrap();
    match db.delete(&id).await {
        Ok(deleted) => match deleted {
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::InternalServerError()
                .body(format!("Could not delete task with id {}", id)),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/expenditure/test/create")]
pub async fn test_create(db: Data<MongoRepository<ExpenditureDocument>>) -> HttpResponse {
    let expenditure = ExpenditureCreate {
        description: "test".to_string(),
        category: Some(Category("test".to_string())),
        cost: Cost {
            amount: 12.,
            periodicity: crate::model::expenditure::Periodicity::Bimonthly,
        },
        ownership: crate::model::expenditure::Ownership::Personal(User("simon".to_string())),
    }
    .into_model(uuid::Uuid::new_v4());
    result_to_response(db.create(&expenditure).await, ExpenditureRead::from_model)
}
