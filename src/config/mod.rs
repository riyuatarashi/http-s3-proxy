use std::env;
use dotenv::dotenv;
use s3::{Bucket, Region, creds::Credentials};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_bucket_name: String,
    pub s3_path_style: bool,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
    pub debug_enabled: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        // Load .env file
        if let Err(err) = dotenv() {
            eprintln!("Warning: Could not load .env file: {}", err);
            eprintln!("Make sure you have a .env file in the project root or set environment variables directly.");
        }

        // Load configuration from environment variables
        let s3_endpoint = env::var("S3_ENDPOINT")
            .map_err(|_| "S3_ENDPOINT environment variable is required".to_string())?
            .trim_end_matches('/').to_string();
        let s3_region = env::var("S3_REGION")
            .unwrap_or_else(|_| "us-east-1".to_string());
        let s3_access_key = env::var("S3_ACCESS_KEY")
            .map_err(|_| "S3_ACCESS_KEY environment variable is required".to_string())?;
        let s3_secret_key = env::var("S3_SECRET_KEY")
            .map_err(|_| "S3_SECRET_KEY environment variable is required".to_string())?;
        let s3_bucket_name = env::var("S3_BUCKET_NAME")
            .map_err(|_| "S3_BUCKET_NAME environment variable is required".to_string())?;
        let s3_path_style = env::var("S3_PATH_STYLE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);
        
        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8089".to_string())
            .parse::<u16>()
            .map_err(|_| "SERVER_PORT must be a valid port number".to_string())?;
        
        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        let debug_enabled = log_level.to_lowercase().contains("debug") || log::log_enabled!(log::Level::Debug);

        Ok(AppConfig {
            s3_endpoint,
            s3_region,
            s3_access_key,
            s3_secret_key,
            s3_bucket_name,
            s3_path_style,
            server_host,
            server_port,
            log_level,
            debug_enabled,
        })
    }

    pub fn create_s3_bucket(&self) -> Result<Bucket, String> {
        let region = Region::Custom {
            region: self.s3_region.clone(),
            endpoint: self.s3_endpoint.clone(),
        };
        
        let credentials = Credentials::new(
            Some(&self.s3_access_key),
            Some(&self.s3_secret_key),
            None, None, None,
        ).map_err(|e| format!("Failed to create S3 credentials: {}", e))?;
        
        let mut bucket = Bucket::new(&self.s3_bucket_name, region, credentials)
            .map_err(|e| format!("Failed to create S3 bucket client: {}", e))?;
        
        if self.s3_path_style {
            bucket = bucket.with_path_style();
        }

        Ok(*bucket)
    }

    pub fn log_startup_info(&self, bucket: &Bucket) {
        println!();
        log::info!("🚀 Starting HTTP S3 Proxy Server");
        log::info!("📡 Listening on: http://{}:{}", self.server_host, self.server_port);
        log::info!("🌐 Public URL: http://localhost:{}", self.server_port);
        log::info!("🪣 S3 Endpoint: {}", self.s3_endpoint);
        log::info!("🗂️  S3 Region: {}", self.s3_region);
        log::info!("📁 S3 Bucket: {}", self.s3_bucket_name);
        log::info!("🛣️  Path Style: {}", self.s3_path_style);
        log::info!("📝 Log Level: {}", self.log_level);
        
        if self.debug_enabled {
            log::info!("🔍 Debug Mode: ENABLED - S3 calls will be logged in detail");
            log::debug!("🔧 Debug Configuration:");
            log::debug!("   └─ S3 Access Key: {}***", &self.s3_access_key[..std::cmp::min(4, self.s3_access_key.len())]);
            log::debug!("   └─ S3 Secret Key: [REDACTED] ({} chars)", self.s3_secret_key.len());
            log::debug!("   └─ Bucket Host: {}", bucket.host());
            log::debug!("   └─ Bucket Name: {}", bucket.name());
            log::debug!("   └─ Bucket Region: {}", bucket.region());
        } else {
            log::info!("🔇 Debug Mode: DISABLED - Set RUST_LOG=debug for detailed S3 logging");
        }
        
        log::info!("✅ Server ready to accept connections!");
        println!();
    }
}
