use crate::error::{MongoErr, Result};

pub async fn collection_names() -> Result<Vec<String>> {
    let db = crate::Mongo::database();

    db.list_collection_names(None)
        .await
        .map_err(Into::<MongoErr>::into)
}
