use mongodb::error::Error;

pub type Result<T, E = MongoErr> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum MongoErr {
    #[error("{0}")]
    Error(#[from] Error),

    #[error("NotFoundError: {0} Not Found")]
    NotFound(String),
}

impl MongoErr {
    pub fn not_found(message: &str) -> Self {
        Self::NotFound(message.into())
    }
}
