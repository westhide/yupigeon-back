// @Author: westhide.yzw
// @Date: 2022-02-22 12:44:32
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-02-22 12:44:32

use poem::{
    handler,
    web::{Query, StaticFileRequest},
    FromRequest, IntoResponse, Request, Response, Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileParams {
    path: String,
    prefer_utf8: Option<bool>,
}

#[handler]
pub async fn file(req: &Request, Query(params): Query<FileParams>) -> Result<Response> {
    let FileParams { path, prefer_utf8 } = params;

    let response = StaticFileRequest::from_request_without_body(req)
        .await?
        .create_response(&path, prefer_utf8.unwrap_or(true))?
        .into_response();
    Ok(response)
}
