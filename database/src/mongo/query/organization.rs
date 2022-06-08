use mongodb::{
    bson::{doc, Document},
    options::FindOneOptions,
};

use crate::mongo::{
    collection::OrganizationCompany,
    common::CollectionTrait,
    error::{MongoErr, Result},
};

pub async fn find_organization_company(
    filter: impl Into<Option<Document>>,
    options: impl Into<Option<FindOneOptions>>,
) -> Result<Option<OrganizationCompany>> {
    OrganizationCompany::collection()?
        .find_one(filter, options)
        .await
        .map_err(Into::<MongoErr>::into)
}

pub async fn organization_company(finance_code: &str) -> Result<Option<OrganizationCompany>> {
    find_organization_company(doc! {"financeCode":finance_code}, None).await
}
