use crate::types::response::ErrorResponse;
use actix_web::{web, FromRequest, HttpResponse, Result};
use otpauth::TOTP;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_INTERVAL: u64 = 60;

#[derive(Debug, Serialize, Deserialize)]
struct CodeRes {
    code: u32,
    interval: u64,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodeReq {
    code: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Interval {
    interval: Option<u64>,
    id: Option<String>,
}

async fn generate_otp(
    query: web::Query<Interval>,
    secret: web::Data<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);

    if timestamp.is_err() {
        return Err(ErrorResponse::ServerError);
    }

    let interval = match query.interval {
        Some(expr) => expr,
        None => DEFAULT_INTERVAL,
    };

    let id = match &query.id {
        Some(x) => x,
        None => "",
    };

    let auth = TOTP::new(format!("{}{}", secret.get_ref(), id));

    let code = auth.generate(interval, timestamp.unwrap().as_secs());

    return Ok(HttpResponse::Ok().json(CodeRes {
        code,
        interval,
        id: id.to_string(),
    }));
}

async fn verify_otp(
    body: Result<web::Json<CodeReq>, <web::Json<CodeReq> as FromRequest>::Error>,
    query: web::Query<Interval>,
    secret: web::Data<String>,
) -> Result<HttpResponse, ErrorResponse> {
    if body.is_err() {
        return Err(ErrorResponse::BadRequest {
            message: body.as_ref().unwrap_err().to_string(),
        });
    }

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);

    if timestamp.is_err() {
        return Err(ErrorResponse::ServerError);
    }

    let unwrapped_body = body.unwrap();

    let interval = match query.interval {
        Some(expr) => expr,
        None => DEFAULT_INTERVAL,
    };

    let id = match &query.id {
        Some(x) => x,
        None => "",
    };

    let auth = TOTP::new(format!("{}{}", secret.get_ref(), id));

    let verified = auth.verify(unwrapped_body.code, interval, timestamp.unwrap().as_secs());

    if !verified {
        return Err(ErrorResponse::Unauthorized {
            message: "Your code is incorrect or has expired.".to_string(),
        });
    }

    return Ok(HttpResponse::NoContent().finish());
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/code")
            .route("", web::get().to(generate_otp))
            .route("/verify", web::put().to(verify_otp)),
    );
}
