use chrono::NaiveDateTime;
use poem::{error::BadRequest, web::Json, Result};
use serde::{Deserialize, Serialize};

pub fn parse_datetime(time_str: &str) -> Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").map_err(BadRequest)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeParams {
    begin_time: String,
    end_time: String,
}

pub trait ParseDateTimeParams {
    fn get_datetime_params(&self) -> Result<(NaiveDateTime, NaiveDateTime)>;
}

impl ParseDateTimeParams for DateTimeParams {
    fn get_datetime_params(&self) -> Result<(NaiveDateTime, NaiveDateTime)> {
        let Self {
            begin_time: begin_time_str,
            end_time: end_time_str,
            ..
        } = self;

        let begin_time = parse_datetime(begin_time_str)?;
        let end_time = parse_datetime(end_time_str)?;

        Ok((begin_time, end_time))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    message: String,
}

impl<T> Response<T> {
    pub fn new(data: Option<T>, message: &str) -> Result<Json<Self>> {
        Ok(Json(Response {
            data,
            message: message.into(),
        }))
    }
}
