use once_cell::sync::OnceCell;
use tokio::sync::{Mutex, MutexGuard};

use crate::service::error::{Result, WrapError};

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

pub fn get_global_data<'a>() -> Result<MutexGuard<'a, GlobalData>> {
    GLOBAL_DATA
        .get()
        .ok_or_else(|| WrapError::message_error("Can Not Get GLOBAL_DATA"))?
        .try_lock()
        .map_err(|_| WrapError::message_error("数据更新中"))
}
