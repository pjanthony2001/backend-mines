use vercel_runtime::{Body, Error, Request, Response, run};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Event {
    title: String,
    category: String,
    date: String,
    time: String,
    location: String,
    description: String,
    image: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the Vercel runtime
    run(handler).await
}

async fn handler(_req: Request) -> Result<Response<Body>, Error> {

    // Handle OPTIONS preflight request
    if _req.method() == "OPTIONS" {
        return Ok(Response::builder()
            .status(204)
            .header("Access-Control-Allow-Origin", "*") // or restrict to your frontend origin
            .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            .header("Access-Control-Allow-Headers", "*")
            .body(Body::Empty)?);
    }


    let events = vec![
        Event {
            title: "Welcome Week Campus Tour".to_string(),
            category: "Social".to_string(),
            date: "September 5, 2025".to_string(),
            time: "2:00 PM - 4:00 PM".to_string(),
            location: "Meet at Maison des Mines".to_string(),
            description: "New to campus? Join us for a comprehensive tour where we show you all the essential spots!".to_string(),
            image: "/mines-tour.jpg".to_string(),
        },
        Event {
            title: "Visa Renewal Workshop".to_string(),
            category: "Admin Support".to_string(),
            date: "June 28, 2025".to_string(),
            time: "5:00 PM - 6:30 PM".to_string(),
            location: "Room V101, Vendome".to_string(),
            description: "Get expert guidance on the visa renewal process. We will cover all the required documents and answer questions.".to_string(),
            image: "/visa-workshop.webp".to_string(),
        },
        Event {
            title: "International Food Festival".to_string(),
            category: "Cultural".to_string(),
            date: "July 12, 2025".to_string(),
            time: "6:00 PM onwards".to_string(),
            location: "Place d'Italie".to_string(),
            description: "Share a dish from your home country and taste flavors from around the world!".to_string(),
            image: "/ian-dumplings.jpg".to_string(),
        },
    ];

    let json = serde_json::to_string(&events)?;
    Ok(Response::builder()
        .header("Access-Control-Allow-Origin", "*") // or your frontend domain for better security
        .header("Content-Type", "application/json")
        .body(Body::Text(json))?)
}
