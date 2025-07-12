use actix_web::{post, web::Payload};

#[post("/buttonclicked")]
pub async fn test_button(body: Payload) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = body.to_bytes().await?;
    let message = String::from_utf8(bytes.into())?;

    #[cfg(feature = "debug-print")]
    println!("Received message {message} from frontend");

    return Ok(());
}
