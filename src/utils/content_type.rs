use std::collections::HashMap;

/// Determines the appropriate content type for a file based on its extension or S3 headers
pub fn determine_content_type(file_path: &str, s3_headers: Option<&HashMap<String, String>>) -> String {
    // First, try to get content type from S3 headers
    if let Some(headers) = s3_headers {
        if let Some(content_type) = headers.get("content-type").or_else(|| headers.get("Content-Type")) {
            log::debug!("🏷️  Using S3-provided Content-Type: {}", content_type);
            return content_type.clone();
        }
    }
    
    // Fallback to determining by file extension
    let extension = std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let content_type = match extension.to_lowercase().as_str() {
        // Text formats
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "md" => "text/markdown",
        
        // Document formats
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        
        // Image formats
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "bmp" => "image/bmp",
        "tiff" | "tif" => "image/tiff",
        
        // Video formats
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "wmv" => "video/x-ms-wmv",
        "flv" => "video/x-flv",
        
        // Audio formats
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "aac" => "audio/aac",
        "flac" => "audio/flac",
        
        // Archive formats
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "bz2" => "application/x-bzip2",
        "7z" => "application/x-7z-compressed",
        "rar" => "application/vnd.rar",
        
        // Font formats
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "eot" => "application/vnd.ms-fontobject",
        
        // Application formats
        "bin" => "application/octet-stream",
        "exe" => "application/x-msdownload",
        "dmg" => "application/x-apple-diskimage",
        "iso" => "application/x-iso9660-image",
        
        // Default fallback
        _ => "application/octet-stream",
    };
    
    log::debug!("🔍 Content-Type determined by extension '{}': {}", extension, content_type);
    content_type.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_content_type_from_extension() {
        assert_eq!(determine_content_type("file.jpg", None), "image/jpeg");
        assert_eq!(determine_content_type("file.png", None), "image/png");
        assert_eq!(determine_content_type("file.html", None), "text/html");
        assert_eq!(determine_content_type("file.css", None), "text/css");
        assert_eq!(determine_content_type("file.js", None), "application/javascript");
        assert_eq!(determine_content_type("file.json", None), "application/json");
        assert_eq!(determine_content_type("file.pdf", None), "application/pdf");
        assert_eq!(determine_content_type("file.unknown", None), "application/octet-stream");
    }

    #[test]
    fn test_content_type_from_s3_headers() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/custom".to_string());
        
        assert_eq!(determine_content_type("file.jpg", Some(&headers)), "text/custom");
    }

    #[test]
    fn test_case_insensitive_extensions() {
        assert_eq!(determine_content_type("file.JPG", None), "image/jpeg");
        assert_eq!(determine_content_type("file.HTML", None), "text/html");
        assert_eq!(determine_content_type("file.PDF", None), "application/pdf");
    }
}
