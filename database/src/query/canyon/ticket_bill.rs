use sea_orm::{entity::prelude::*, InsertResult};

pub async fn insert_many<E, A>(models: Vec<E::Model>) -> Result<InsertResult<A>, DbErr>
where
    E: EntityTrait,
    A: ActiveModelTrait<Entity = E> + From<E::Model>,
{
    let txn = crate::Database::new("default").await?.txn;

    let records: Vec<A> = models.iter().map(|model| model.to_owned().into()).collect();

    let result = E::insert_many(records).exec(&txn).await?;

    txn.commit().await?;
    Ok(result)
}
