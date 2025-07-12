pub mod handlers;
pub mod index;

use actix_files::NamedFile;
use actix_web::{App, Error, HttpRequest, HttpServer, get};
use handlers::test_button;
use index::{locate, page};

const PORT: u16 = 40317;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(index_redirect)
            .service(page)
            .service(test_button)
    })
    .bind(("127.0.0.1", PORT))?
    .run();

    println!("Listening on http://127.0.0.1:{PORT}");

    return server.await;
}

#[get("/")]
async fn index_redirect(_req: HttpRequest) -> Result<NamedFile, Error> {
    return Ok(locate("index")?.use_last_modified(true));
}
