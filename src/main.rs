use actix_web::{middleware, web, App, HttpServer};
mod routes;
mod types;
use std::env;
use otpauth::TOTP;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port = env::var("OTP_SERVER_PORT").unwrap_or("30624".to_string());

    let secret = env::var("OTP_SERVER_SECRET").expect("OTP_SERVER_SECRET is not set.");

    let generator = web::Data::new(TOTP::new(secret));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(generator.clone())
            .configure(routes::code::setup)
            .configure(routes::health_check::setup)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
