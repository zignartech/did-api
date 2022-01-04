#![allow(non_snake_case)]
use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
mod app_service;
mod jsons_did;
mod utils_did;
use crate::app_service::createDidApi;
use crate::app_service::createVcApi;
use crate::app_service::createVpApi;
use crate::app_service::removeVmApi;
use crate::app_service::verifyValidityApiCred;
use crate::app_service::verifyValidityApiPres;
use crate::app_service::addVerifMethodApi;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init env vars
    dotenv::from_path("./development.env").ok();

    // building address and ip
    let port = std::env::var("PORT_API").unwrap_or("8080".to_string());
    let host = std::env::var("HOST_API").unwrap_or("127.0.0.1".to_string());
    let address = format!("{}:{}", host, port);

    println!("API => {}", address);

    // building host frontend
    let host_cors = std::env::var("HOST_CORS").unwrap_or("127.0.0.1".to_string());

    println!("Frontend Cors => {}", host_cors);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&host_cors)
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(createDidApi)
            .service(createVcApi)
            .service(createVpApi)
            .service(verifyValidityApiCred)
            .service(verifyValidityApiPres)
            .service(removeVmApi)
            .service(addVerifMethodApi)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!(
            "🔥🔥🔥 Couldn't start the server in port {}: {:?}",
            port, err
        )
    })
    .run()
    .await?;
    Ok(server)
}
