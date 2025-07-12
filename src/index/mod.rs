use actix_files::NamedFile;
use actix_web::{Error, HttpRequest, get};

const NOT_FOUND_PAGE_PATH: &'static str = "static/404.html";

pub fn locate(filename: &str) -> Result<NamedFile, Error> {
    let mut file_path = String::from("frontend/out/") + filename;

    if !filename.contains(".") {
        file_path += ".html";
    }

    #[cfg(feature = "debug-print")]
    println!("searching for {filename} at {file_path}...");

    return Ok(NamedFile::open(file_path).unwrap_or(NamedFile::open(NOT_FOUND_PAGE_PATH)?));
}

#[get("/{filename:.*}")]
pub async fn page(req: HttpRequest) -> Result<NamedFile, Error> {
    let file = req.match_info().query("filename");
    return Ok(locate(file)?.use_last_modified(true));
}
