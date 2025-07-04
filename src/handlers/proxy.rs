use actix_web::{get, web, HttpResponse, Responder};
use s3::bucket::Bucket;
use crate::utils::determine_content_type;

pub struct AppState {
    pub bucket: Bucket,
}

#[get("/{file_path:.*}")]
pub async fn proxy_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let file_path = path.into_inner();
    log::info!("📥 Received request for file: {}", file_path);
    
    // Debug logging for S3 call details
    log::debug!("🔍 S3 Debug Info:");
    log::debug!("   └─ Bucket: {}", data.bucket.name());
    log::debug!("   └─ Region: {}", data.bucket.region());
    log::debug!("   └─ Endpoint: {}", data.bucket.host());
    log::debug!("   └─ Object Key: {}", file_path);
    log::debug!("   └─ Path Style: {}", data.bucket.is_path_style());
    
    log::debug!("🌐 Making S3 GetObject call for: {}", file_path);
    let start_time = std::time::Instant::now();
    
    match data.bucket.get_object(&file_path).await {
        Ok(response) => {
            let duration = start_time.elapsed();
            let content_length = response.bytes().len();
            
            log::info!("✅ Successfully served file: {} ({} bytes)", file_path, content_length);
            log::debug!("⏱️  S3 call completed in: {:?}", duration);
            log::debug!("📊 Response details:");
            log::debug!("   └─ Status Code: {}", response.status_code());
            log::debug!("   └─ Content Length: {} bytes", content_length);
            
            // Log response headers in debug mode
            let headers = response.headers();
            log::debug!("📋 S3 Response Headers:");
            for (key, value) in &headers {
                log::debug!("   └─ {}: {}", key, value);
            }
            
            // Determine content type based on file extension or S3 response
            let content_type = determine_content_type(&file_path, Some(&headers));
            log::debug!("📄 Content-Type determined as: {}", content_type);
            
            HttpResponse::Ok()
                .content_type(content_type)
                .body(response.bytes().clone())
        }
        Err(err) => {
            let duration = start_time.elapsed();
            log::warn!("❌ File not found: {} - Error: {:?}", file_path, err);
            log::debug!("⏱️  Failed S3 call completed in: {:?}", duration);
            log::debug!("🔍 Error details: {:#?}", err);
            
            HttpResponse::NotFound().body("File not found")
        }
    }
}
