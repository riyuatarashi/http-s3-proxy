use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;

use crate::config::AppConfig;
use crate::handlers::{AppState, proxy_handler};

pub struct Application {
    config: AppConfig,
}

impl Application {
    pub fn new() -> Result<Self, String> {
        // Initialize logger first
        let config = AppConfig::from_env()?;
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
        
        Ok(Application { config })
    }

    pub async fn run(self) -> std::io::Result<()> {
        // Create S3 bucket client
        let bucket = self.config.create_s3_bucket()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        // Log startup information
        self.config.log_startup_info(&bucket);

        // Create application state
        let app_state = web::Data::new(AppState { bucket });

        // Start HTTP server
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(app_state.clone())
                .service(proxy_handler)
        })
        .bind((self.config.server_host.as_str(), self.config.server_port))?
        .run()
        .await
    }
}
