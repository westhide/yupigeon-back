use crate::mongo::{
    error::{MongoErr, Result},
    MongoPool,
};

pub async fn collection_names() -> Result<Vec<String>> {
    let db = MongoPool::database()?;

    db.list_collection_names(None)
        .await
        .map_err(Into::<MongoErr>::into)
}
