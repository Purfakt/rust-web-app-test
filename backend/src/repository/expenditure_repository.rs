use crate::model::expenditure::Expenditure;
use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;

use super::{expenditure_document::ExpenditureDocument, mongo_repository::MongoRepository};

impl MongoRepository<ExpenditureDocument> {
    pub async fn create(&self, expenditure: &Expenditure) -> Result<Option<Expenditure>> {
        let expenditure_doc: ExpenditureDocument = expenditure.into();
        self.collection.insert_one(expenditure_doc, None).await?;
        self.get_one(&expenditure.id).await
    }

    pub async fn get_one(&self, id: &uuid::Uuid) -> Result<Option<Expenditure>> {
        let bson_id = bson::Uuid::from_bytes(*id.as_bytes());

        let expenditure_document = self.collection.find_one(doc! {"_id": bson_id}, None).await;
        expenditure_document
            .map(|option| option.map(|doc| Expenditure::from(&doc)))
            .map_err(|err| err.into())
    }

    pub async fn get_all(&self) -> Result<Vec<Expenditure>> {
        self.collection
            .find(None, None)
            .await?
            .try_collect::<Vec<ExpenditureDocument>>()
            .await
            .map(|vec| vec.iter().map(Expenditure::from).collect())
            .map_err(|err| err.into())
    }

    pub async fn update(
        &self,
        id: &uuid::Uuid,
        expenditure: &Expenditure,
    ) -> Result<Option<Expenditure>> {
        let bson_id = bson::Uuid::from_bytes(*id.as_bytes());
        let doc = ExpenditureDocument::from(expenditure);
        self.collection
            .update_one(
                doc! {"_id": bson_id},
                doc! {"$set": bson::to_bson(&doc)?},
                None,
            )
            .await?;
        self.get_one(&expenditure.id).await
    }

    pub async fn delete(&self, id: &uuid::Uuid) -> Result<bool> {
        let bson_id = bson::Uuid::from_bytes(*id.as_bytes());

        self.collection
            .delete_one(doc! {"_id": bson_id}, None)
            .await
            .map(|delete_result| delete_result.deleted_count == 1)
            .map_err(|err| err.into())
    }
}
