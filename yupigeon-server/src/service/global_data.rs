use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct GlobalData {
    pub is_ship_ticket_bill_refresh: bool,
}

pub static GLOBAL_DATA: OnceCell<Mutex<GlobalData>> = OnceCell::new();

pub fn init_global_data() {
    let global_data = Mutex::new(GlobalData {
        is_ship_ticket_bill_refresh: false,
    });

    GLOBAL_DATA
        .set(global_data)
        .expect("Can not set global_data");
}
