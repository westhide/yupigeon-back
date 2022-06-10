use once_cell::sync::OnceCell;
use oracle::{Connection, Error, Result};

pub struct OracleDatabase {
    connection: Connection,
}

pub static ORACLE_DATABASE: OnceCell<OracleDatabase> = OnceCell::new();

use crate::config::get_config;

fn get_config_by_key(key: &str) -> Result<String> {
    get_config(key).map_err(|e| Error::InvalidBindName(e.to_string()))
}

impl OracleDatabase {
    pub fn init() -> Result<()> {
        let username = get_config_by_key("ORACLE_UESERNAME")?;
        let password = get_config_by_key("ORACLE_PASSWORD")?;
        let url = get_config_by_key("ORACLE_URL")?;
        let connection = Connection::connect(username, password, url)?;
        let oracle_database = Self { connection };
        ORACLE_DATABASE
            .set(oracle_database)
            .map_err(|_| Error::InvalidBindName("Can Not Set MONGO_POOL twice".to_string()))
    }

    pub fn connection<'a>() -> Result<&'a Connection> {
        match ORACLE_DATABASE.get() {
            Some(oracle_database) => Ok(&oracle_database.connection),
            None => Err(Error::InvalidBindName(
                "ORACLE_DATABASE Not Found".to_string(),
            )),
        }
    }
}
