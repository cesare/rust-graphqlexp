use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
    });
    server.bind("127.0.0.1:8080")?.run().await?;

    Ok(())
}
