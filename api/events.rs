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
    let events = vec![
        Event {
            title: "Welcome Week Campus Tour".to_string(),
            category: "Social".to_string(),
            date: "September 5, 2024".to_string(),
            time: "2:00 PM - 4:00 PM".to_string(),
            location: "Meet at Student Union Entrance".to_string(),
            description: "New to campus? Join us for a comprehensive tour where we show you all the essential spots!".to_string(),
            image: "/mines-tour.jpg".to_string(),
        },
        Event {
            title: "Visa Renewal Workshop".to_string(),
            category: "Admin Support".to_string(),
            date: "September 12, 2024".to_string(),
            time: "5:00 PM - 6:30 PM".to_string(),
            location: "Room 301, Admin Building".to_string(),
            description: "Get expert guidance on the visa renewal process. We will cover all the required documents and answer questions.".to_string(),
            image: "/visa-workshop.webp".to_string(),
        },
        Event {
            title: "International Food Festival".to_string(),
            category: "Cultural".to_string(),
            date: "September 20, 2024".to_string(),
            time: "6:00 PM onwards".to_string(),
            location: "Main University Lawn".to_string(),
            description: "Share a dish from your home country and taste flavors from around the world!".to_string(),
            image: "/ian-dumplings.jpg".to_string(),
        },
    ];

    let json = serde_json::to_string(&events)?;
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::Text(json))?)
}
