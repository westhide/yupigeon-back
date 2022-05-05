use mongodb::error::Result;

pub async fn collection_names() -> Result<Vec<String>> {
    let db = crate::Mongo::database().await?;

    let collection_names = db.list_collection_names(None).await?;
    Ok(collection_names)
}
