use mongodb::error::Error;

pub type Result<T, E = MongoErr> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum MongoErr {
    #[error("{0}")]
    Error(#[from] Error),

    #[error("MongodbMessage: {0}")]
    Message(String),
}

impl MongoErr {
    pub fn message_error(message: &str) -> Self {
        Self::Message(message.into())
    }
}
