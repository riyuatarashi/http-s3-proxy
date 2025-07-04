mod config;
mod handlers;
mod utils;
mod app;

use app::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = Application::new()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    app.run().await
}
