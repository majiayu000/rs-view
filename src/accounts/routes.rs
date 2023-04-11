use crate::accounts::{Accounts};
use crate::error_handler::AccountError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/accounts")]
async fn find_all() -> Result<HttpResponse, AccountError> {
    let accounts = web::block(|| Accounts::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(accounts))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
}