use oracle::{sql_type::ToSql, Result, RowValue};

use crate::oracle::OracleDatabase;

pub trait QueryTrait {
    fn query(sql: &str, params: &[&dyn ToSql]) -> Result<Vec<Self>>
    where
        Self: RowValue,
    {
        let rows = OracleDatabase::connection()?.query_as::<Self>(sql, params)?;

        let mut data = vec![];
        for row in rows {
            data.push(row?);
        }

        Ok(data)
    }
}

impl<T: RowValue> QueryTrait for T {}
