# HTTP S3 Proxy

https://github.com/riyuatarashi/http-s3-proxy/actions/workflows/rust.yml/badge.svg

A simple HTTP proxy server that serves files from S3-compatible storage services.

## Features

- 🚀 Fast HTTP proxy for S3-compatible storage
- 📁 Serves files directly from S3 buckets
- 🔧 Configurable via environment variables
- 📝 Request logging with detailed information
- 🛣️ Support for both path-style and virtual-hosted-style URLs

## Configuration

The application is configured using environment variables. You can either:

1. Create a `.env` file in the project root (recommended for development)
2. Set environment variables directly (recommended for production)

### Required Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `S3_ENDPOINT` | S3-compatible endpoint URL | `https://s3.amazonaws.com` |
| `S3_ACCESS_KEY` | S3 access key ID | `your-access-key` |
| `S3_SECRET_KEY` | S3 secret access key | `your-secret-key` |
| `S3_BUCKET_NAME` | Name of the S3 bucket | `my-files-bucket` |

### Optional Environment Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `S3_REGION` | S3 region | `us-east-1` | `eu-west-1` |
| `S3_PATH_STYLE` | Use path-style URLs | `true` | `false` |
| `SERVER_HOST` | Server bind address | `0.0.0.0` | `127.0.0.1` |
| `SERVER_PORT` | Server port | `8089` | `3000` |
| `RUST_LOG` | Log level | `info` | `debug` |

## Setup

1. **Clone and build the project:**
   ```bash
   git clone <repository-url>
   cd http-s3-proxy
   cargo build --release
   ```

2. **Create your `.env` file:**
   ```bash
   cp .env.example .env
   # Edit .env with your actual S3 credentials and settings
   ```

3. **Run the server:**
   ```bash
   ./target/release/http-s3-proxy
   ```

## Usage

Once the server is running, you can access files from your S3 bucket via HTTP:

```
http://localhost:8089/path/to/your/file.txt
```

The server will:
- Log each incoming request
- Fetch the file from your S3 bucket
- Return the file content with appropriate headers
- Log the result (success with file size or error)

## Example .env file

```env
# S3 Configuration
S3_ENDPOINT=https://minio.example.com
S3_REGION=us-east-1
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin123
S3_BUCKET_NAME=my-public-files
S3_PATH_STYLE=true

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8089

# Logging
RUST_LOG=info
```

## Logging

The application provides detailed logging including:

- 🚀 Server startup information
- 📡 Listening address and port
- 🌐 Public URL
- 🪣 S3 configuration details
- 📝 Request logging for each file access
- ⚠️ Error logging for missing files

### Debug Logging

When `RUST_LOG=debug` is set, the application provides extensive S3 debugging information:

- 🔍 Detailed S3 client configuration
- 🌐 Complete S3 API call information
- ⏱️ Response times for each S3 operation
- 📋 S3 response headers
- 📄 Content-type determination logic
- 🔧 Bucket and authentication details (with sensitive data redacted)

Example debug output:
```
🔍 S3 Debug Info:
   └─ Bucket: my-bucket
   └─ Region: us-east-1
   └─ Endpoint: https://s3.amazonaws.com
   └─ Object Key: path/to/file.jpg
   └─ Path Style: true
🌐 Making S3 GetObject call for: path/to/file.jpg
⏱️  S3 call completed in: 234ms
📋 S3 Response Headers:
   └─ content-type: image/jpeg
   └─ content-length: 52431
   └─ last-modified: Wed, 21 Oct 2015 07:28:00 GMT
```

## Security Notes

- Never commit your `.env` file to version control
- Use strong, unique access keys for production
- Consider using IAM roles or temporary credentials in production
- Limit S3 bucket permissions to only what's necessary

## Code Architecture

The application is structured with a modular architecture for improved maintainability:

```
src/
├── main.rs              # Application entry point
├── app.rs               # Main application setup and server initialization  
├── config/              # Configuration management
│   └── mod.rs          # Environment variable loading and S3 client setup
├── handlers/            # HTTP request handlers
│   ├── mod.rs          # Handler module exports
│   └── proxy.rs        # S3 proxy endpoint logic
└── utils/               # Utility functions
    ├── mod.rs          # Utility module exports
    └── content_type.rs # MIME type detection logic
```

### Key Components:

- **`config`**: Handles environment variable loading, configuration validation, and S3 client creation
- **`handlers`**: Contains HTTP request handlers with detailed logging and error handling
- **`utils`**: Provides utility functions like content-type detection with extensive file format support
- **`app`**: Orchestrates the application startup and server configuration

## Building for Production

```bash
cargo build --release
```

The binary will be available at `./target/release/http-s3-proxy`.

## Running Tests

```bash
cargo test
```

## Docker Support

You can also run this in a Docker container by setting environment variables:

```bash
docker run -d \
  -p 8089:8089 \
  -e S3_ENDPOINT=https://your-s3-endpoint.com \
  -e S3_ACCESS_KEY=your-access-key \
  -e S3_SECRET_KEY=your-secret-key \
  -e S3_BUCKET_NAME=your-bucket \
  http-s3-proxy
```
