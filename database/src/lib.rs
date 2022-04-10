// @Author: westhide.yzw
// @Date: 2022-03-20 16:15:05
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-20 16:15:05

mod config;
mod database;
pub mod entity;
pub mod ship_ticket_bill;
pub mod tenpay_bill;

pub use database::Database;
pub use sea_orm;
