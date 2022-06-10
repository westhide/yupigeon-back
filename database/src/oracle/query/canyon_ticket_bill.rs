use oracle::Result;

use crate::oracle::OracleDatabase;

pub fn test_oracle() -> Result<()> {
    let sql = "SELECT count(*) AS cou FROM XZDXG.WEB_ORDER_CHECK_LOG";

    let rows = OracleDatabase::connection()?.query_as::<i32>(sql, &[])?;
    for row in rows {
        let cou = row?;
        println!("=============={:14}", cou,);
    }
    Ok(())
}
