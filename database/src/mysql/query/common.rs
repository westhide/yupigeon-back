use async_trait::async_trait;
use sea_orm::{
    entity::prelude::*, ActiveModelTrait, InsertResult, IntoActiveModel, Iterable, Value,
};

#[async_trait]
pub trait QueryTrait {
    async fn insert_once<A>(models: Vec<Self::Model>) -> Result<(), DbErr>
    where
        Self: EntityTrait,
        Self::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = Self> + ActiveModelBehavior + Send,
    {
        let txn = crate::mysql::Database::new("default").await?.txn;

        for model in models {
            let active_model: A = model.into_active_model();
            active_model.insert(&txn).await?;
        }

        txn.commit().await
    }

    async fn insert_many<A>(models: Vec<Self::Model>) -> Result<InsertResult<A>, DbErr>
    where
        Self: EntityTrait,
        Self::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = Self> + Send,
    {
        let txn = crate::mysql::Database::new("default").await?.txn;

        let records: Vec<A> = models
            .iter()
            .map(|model| model.to_owned().into_active_model())
            .collect();

        let result = <Self as EntityTrait>::insert_many(records)
            .exec(&txn)
            .await?;

        txn.commit().await?;
        Ok(result)
    }

    async fn insert_many_by_chunks<A>(
        models: Vec<Self::Model>,
        chunk_size: usize,
    ) -> Result<(), DbErr>
    where
        Self: EntityTrait,
        Self::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = Self> + Send + std::marker::Sync,
    {
        let txn = crate::mysql::Database::new("default").await?.txn;

        let records: Vec<A> = models
            .iter()
            .map(|model| model.to_owned().into_active_model())
            .collect();

        for chunk in records.chunks(chunk_size) {
            let data = chunk.to_vec();
            <Self as EntityTrait>::insert_many(data).exec(&txn).await?;
        }

        txn.commit().await
    }

    async fn replace_many<A>(models: Vec<Self::Model>) -> Result<(), DbErr>
    where
        Self: EntityTrait,
        Self::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = Self> + ActiveModelBehavior + Send,
    {
        let txn = crate::mysql::Database::new("default").await?.txn;

        for model in models {
            let mut active_model: A = model.into_active_model();
            let mut is_update = true;

            for pk in Self::PrimaryKey::iter() {
                let col = pk.into_column();
                if let Some(pk_value) = active_model.get(col).into_value() {
                    if pk_value == Value::Unsigned(Some(0)) {
                        is_update = false;
                        break;
                    }
                }
            }

            if is_update {
                for col in Self::Column::iter() {
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
}

impl<E: EntityTrait> QueryTrait for E {}
