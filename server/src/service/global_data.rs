use std::collections::HashMap;

use once_cell::sync::OnceCell;
use serde::Serialize;
use tokio::sync::{broadcast, Mutex, MutexGuard};

use crate::service::error::{Result, WrapError};

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShipTicketRefreshStatus {
    pub is_refresh: bool,
    pub last_refresh_datetime: String,
}

type WebSocketSender = broadcast::Sender<String>;
type WebSocketBook = HashMap<String, WebSocketSender>;

#[derive(Debug)]
pub struct GlobalData {
    pub ship_ticket_refresh_status: Mutex<ShipTicketRefreshStatus>,
    pub websocket_sender_book: Mutex<WebSocketBook>,
}

pub static GLOBAL_DATA: OnceCell<GlobalData> = OnceCell::new();

pub fn init_global_data() {
    let ship_ticket_refresh_status = Mutex::new(ShipTicketRefreshStatus {
        is_refresh: false,
        last_refresh_datetime: String::from(""),
    });
    let websocket_sender_book = Mutex::new(HashMap::new());

    let global_data = GlobalData {
        ship_ticket_refresh_status,
        websocket_sender_book,
    };

    GLOBAL_DATA.set(global_data).ok();
}

pub fn get_global_data<'a>() -> Result<&'a GlobalData> {
    match GLOBAL_DATA.get() {
        Some(data) => Ok(data),
        None => Err(WrapError::message_error("Can not get GLOBAL_DATA")),
    }
}

impl ShipTicketRefreshStatus {
    fn try_lock<'a>() -> Result<MutexGuard<'a, Self>> {
        match get_global_data()?.ship_ticket_refresh_status.try_lock() {
            Ok(guard) => Ok(guard),
            Err(_) => Err(WrapError::message_error(
                "Can not get GLOBAL_DATA.ship_ticket_refresh_status",
            )),
        }
    }

    pub fn get() -> Result<Self> {
        let mutex_guard = Self::try_lock()?;

        let refresh_status = Self {
            is_refresh: mutex_guard.is_refresh,
            last_refresh_datetime: mutex_guard.last_refresh_datetime.clone(),
        };

        Ok(refresh_status)
    }

    pub fn set(is_refresh: bool, datetime: Option<String>) -> Result<bool> {
        let mut mutex_guard = Self::try_lock()?;

        mutex_guard.is_refresh = is_refresh;
        if let Some(last_refresh_datetime) = datetime {
            mutex_guard.last_refresh_datetime = last_refresh_datetime
        };

        Ok(true)
    }
}

fn try_lock_websocket_sender_book<'a>() -> Result<MutexGuard<'a, WebSocketBook>> {
    match get_global_data()?.websocket_sender_book.try_lock() {
        Ok(guard) => Ok(guard),
        Err(_) => Err(WrapError::message_error(
            "Can not get GLOBAL_DATA.websocket_sender_book",
        )),
    }
}

pub fn get_websocket_sender(name: &str) -> Result<WebSocketSender> {
    let websocket_sender_book = try_lock_websocket_sender_book()?;
    match websocket_sender_book.get(name) {
        Some(sender) => Ok(sender.to_owned()),
        None => Err(WrapError::message_error(&format!(
            "Can not get GLOBAL_DATA.websocket_sender: name='{}'",
            name
        ))),
    }
}

pub fn set_websocket_sender(name: String) -> Result<WebSocketSender> {
    let mut websocket_sender_book = try_lock_websocket_sender_book()?;
    let sender = broadcast::channel::<String>(32).0;
    websocket_sender_book.insert(name, sender.clone());
    Ok(sender)
}
