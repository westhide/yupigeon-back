use futures_util::{SinkExt, StreamExt};
use poem::{
    handler,
    web::{
        websocket::{Message, WebSocket},
        Json,
    },
    IntoResponse,
};

use crate::{
    global_data::{get_websocket_sender, set_websocket_sender},
    service::{error::Result, response::Response},
};

#[handler]
pub fn websocket(ws: WebSocket) -> Result<impl IntoResponse> {
    let sender = set_websocket_sender("default".to_string())?;
    let mut receiver = sender.subscribe();

    let res = ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();

        tokio::spawn(async move {
            while let Some(Ok(message)) = stream.next().await {
                if let Message::Text(text) = message {
                    if sender.send(text).is_err() {
                        break;
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Ok(text) = receiver.recv().await {
                if sink.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
        });
    });

    Ok(res)
}

#[handler]
pub fn broadcast_message(Json(data): Json<Response>) -> Result<impl IntoResponse> {
    let message = serde_json::json!(data).to_string();

    let sender = get_websocket_sender("default")?;
    sender.send(message).ok();

    Response::message("Web socket broadcast success")
}
