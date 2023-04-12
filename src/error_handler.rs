use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::{
    Deserialize,
    Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl AccountError {
    pub fn new(error_status_code: u16, error_message: String) -> AccountError {
        AccountError {
            error_status_code,
            error_message,
        }
    }
}

impl From<DieselError> for AccountError {
    fn from(error: DieselError) -> AccountError {
        match error {
            DieselError::DatabaseError(_, err) => AccountError::new(409, err.message().to_string()),
            DieselError::NotFound => {
                AccountError::new(404, "The employee record not found".to_string())
            }
            err => AccountError::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}


impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.error_status_code, self.error_message)
    }
}

impl ResponseError for AccountError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.error_status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };


        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({"message": error_message}))
        } 
}
