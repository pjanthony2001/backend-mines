use vercel_runtime::{Body, Error, Request, Response, run};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ContactForm {
    name: String,
    email: String,
    subject: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {

    if req.method() == "OPTIONS" {
        return Ok(Response::builder()
            .status(204)
            .header("Access-Control-Allow-Origin", "*") // or restrict to your frontend origin
            .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            .header("Access-Control-Allow-Headers", "*")
            .body(Body::Empty)?);
    }

    let body = req.body();
    let contact: ContactForm = serde_json::from_slice(body)?;

    println!("->> CONTACT FORM RECEIVED:");
    println!("  ->> Name: {}", contact.name);
    println!("  ->> Email: {}", contact.email);
    println!("  ->> Subject: {}", contact.subject);
    println!("  ->> Message: {}", contact.message);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .body(Body::Empty)?)
}
