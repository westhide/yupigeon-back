use sea_orm::{
    entity::prelude::*, ActiveModelTrait, InsertResult, IntoActiveModel, Iterable, Value,
};

pub async fn insert_many<E, A>(models: Vec<E::Model>) -> Result<InsertResult<A>, DbErr>
where
    E: EntityTrait,
    E::Model: IntoActiveModel<A>,
    A: ActiveModelTrait<Entity = E>,
{
    let txn = crate::Database::new("default").await?.txn;

    let records: Vec<A> = models
        .iter()
        .map(|model| model.to_owned().into_active_model())
        .collect();

    let result = E::insert_many(records).exec(&txn).await?;

    txn.commit().await?;
    Ok(result)
}

pub async fn replace_many<E, A>(models: Vec<E::Model>) -> Result<(), DbErr>
where
    E: EntityTrait,
    E::Model: IntoActiveModel<A>,
    A: ActiveModelTrait<Entity = E> + ActiveModelBehavior + std::marker::Send,
{
    let txn = crate::Database::new("default").await?.txn;

    for model in models {
        let mut active_model: A = model.into_active_model();
        let mut is_update = true;

        for pk in E::PrimaryKey::iter() {
            let col = pk.into_column();
            if let Some(pk_value) = active_model.get(col).into_value() {
                if pk_value == Value::Unsigned(Some(0)) {
                    is_update = false;
                    break;
                }
            }
        }

        if is_update {
            for col in E::Column::iter() {
                if let Some(value) = active_model.get(col).into_value() {
                    active_model.set(col, value);
                }
            }
            active_model.update(&txn).await?;
        } else {
            active_model.insert(&txn).await?;
        }
    }

    txn.commit().await
}
