use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponseBody {
    pub message: String,
}

#[derive(Debug, Display, Error)]
pub enum ErrorResponse {
    BadRequest { message: String },
    Unauthorized { message: String },
    #[allow(dead_code)]
    ServerError,
}

fn build_response(res: &ErrorResponse, message: String) -> HttpResponse {
    return HttpResponseBuilder::new(res.status_code()).json(ErrorResponseBody {
        message: message.to_string(),
    });
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match &*self {
            ErrorResponse::BadRequest { message } => {
                return build_response(self, message.to_string())
            }
            
            ErrorResponse::Unauthorized { message } => {
                return build_response(self, message.to_string())
            }

            ErrorResponse::ServerError => {
                return build_response(self, "Internal server error".to_string())
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match &*self {
            ErrorResponse::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ErrorResponse::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ErrorResponse::ServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
