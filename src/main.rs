mod api_error;
mod handlers;

use anyhow::{anyhow, Result};
use log::info;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();
    info!("Starting server");

    let port = std::env::var("PORT")?.parse::<u16>()?;
    let api_prefix = std::env::var("API_PREFIX").unwrap_or("".to_string());

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(actix_web::web::scope(&api_prefix).configure(handlers::config_app))
    });

    #[cfg(feature = "tls")]
    let server = server.bind_openssl(("0.0.0.0", port), {
        info!("Using TLS");
        let mut builder =
            openssl::ssl::SslAcceptor::mozilla_intermediate(openssl::ssl::SslMethod::tls())
                .unwrap();
        let private_key_path = std::env::var("PRIVATE_KEY_PATH")?;
        let certificate_path = std::env::var("CERTIFICATE_PATH")?;
        builder
            .set_private_key_file(private_key_path, openssl::ssl::SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file(certificate_path)
            .unwrap();
        builder
    })?;

    #[cfg(not(feature = "tls"))]
    let server = server.bind(("0.0.0.0", port))?;

    server.run().await.map_err(|e| anyhow!(e))
}
