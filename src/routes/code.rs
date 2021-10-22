use crate::types::response::ErrorResponse;
use actix_web::{web, FromRequest, HttpResponse, Result};
use otpauth::{HOTP, TOTP};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_INTERVAL: u64 = 60;

#[derive(Debug, Serialize, Deserialize)]
struct Code {
    code: u32,
    interval: Option<u64>
}

#[derive(Debug, Serialize, Deserialize)]
struct Interval {
    interval: u64,
}

async fn generate_otp(
    body: Option<web::Json<Interval>>,
    auth: web::Data<TOTP>,
) -> Result<HttpResponse, ErrorResponse> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);

    if timestamp.is_err() {
        return Err(ErrorResponse::ServerError);
    }

    let interval = if body.is_some() {
        body.unwrap().interval
    } else {
        DEFAULT_INTERVAL
    };

    let code = auth.generate(interval, timestamp.unwrap().as_secs());
    return Ok(HttpResponse::Ok().json(Code { code, interval: Some(interval) }));
}

async fn verify_otp(
    body: Result<web::Json<Code>, <web::Json<Code> as FromRequest>::Error>,
    auth: web::Data<TOTP>,
) -> Result<HttpResponse, ErrorResponse> {
    if body.is_err() {
        return Err(ErrorResponse::ClientError {
            message: body.as_ref().unwrap_err().to_string(),
        });
    }
    
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);

    if timestamp.is_err() {
        return Err(ErrorResponse::ServerError);
    }

    let unwrapped_body = body.unwrap();

    let interval = if unwrapped_body.interval.is_some() {
        unwrapped_body.interval.unwrap()
    } else {
        DEFAULT_INTERVAL
    };
    
    let verified = auth.verify(unwrapped_body.code, interval, timestamp.unwrap().as_secs());

    if !verified {
        return Err(ErrorResponse::ClientError {
            message: "Your code has expired.".to_string()
        });
    }

    return Ok(HttpResponse::NoContent().finish());
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/generate").route(web::post().to(generate_otp)));

    cfg.service(web::resource("/verify").route(web::post().to(verify_otp)));
}
