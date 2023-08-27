mod api_error;
mod handlers;

use anyhow::{anyhow, Result};
use log::info;

// TODO tests
// TODO config
// TODO /hello/:name

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting server");

    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(actix_web::web::scope("/api/v1").configure(handlers::config_app))
    });

    #[cfg(feature = "tls")]
    let server = server.bind_openssl(("0.0.0.0", 443), {
        info!("Using TLS");
        let mut builder =
            openssl::ssl::SslAcceptor::mozilla_intermediate(openssl::ssl::SslMethod::tls())
                .unwrap();
        builder
            .set_private_key_file("certs/key.pem", openssl::ssl::SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file("certs/cert.pem")
            .unwrap();
        builder
    })?;

    #[cfg(not(feature = "tls"))]
    let server = server.bind(("0.0.0.0", 80))?;

    server.run().await.map_err(|e| anyhow!(e))
}
