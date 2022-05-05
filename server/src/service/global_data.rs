use once_cell::sync::OnceCell;
use poem::Result;
use tokio::sync::{Mutex, MutexGuard};

use crate::service::error::MessageError;

#[derive(Debug, Clone)]
pub struct GlobalData {
    pub is_ship_ticket_bill_refresh: bool,
    pub last_refresh_datetime: Option<String>,
}

pub static GLOBAL_DATA: OnceCell<Mutex<GlobalData>> = OnceCell::new();

pub fn init_global_data() {
    let global_data = Mutex::new(GlobalData {
        is_ship_ticket_bill_refresh: false,
        last_refresh_datetime: None,
    });

    GLOBAL_DATA
        .set(global_data)
        .expect("Can not set global_data");
}

pub fn get_global_data() -> Result<MutexGuard<'static, GlobalData>> {
    GLOBAL_DATA
        .get()
        .ok_or_else(|| MessageError::new("Can Not Get GLOBAL_DATA"))?
        .try_lock()
        .map_err(|_| MessageError::new("数据更新中").into())
}
