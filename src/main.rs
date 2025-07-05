use actix_web::{ App, get, HttpResponse, HttpServer, main, post, Responder, guard, rt };
use actix_web::web::{ Data, Form, Json, Path, resource, post as web_post };
use serde::{ Deserialize, Serialize };
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct Subscriber {
    name: String,
    email: String,
}

#[get("/healthz")]
async fn health_check() -> impl Responder {
    "OK"
}

#[get("/")]
async fn index() -> HttpResponse {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Contact Form</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 600px; margin: 50px auto; padding: 20px; }
        form { background: #f4f4f4; padding: 20px; border-radius: 8px; }
        label { display: block; margin-bottom: 5px; font-weight: bold; }
        input { width: 100%; padding: 10px; margin-bottom: 15px; border: 1px solid #ddd; border-radius: 4px; }
        button { background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }
        button:hover { background: #0056b3; }
    </style>
</head>
<body>
    <h1>Contact Form</h1>
    <form action="/subscribe" method="post">
        <label for="name">Name:</label>
        <input type="text" id="name" name="name" required>
        
        <label for="email">Email:</label>
        <input type="email" id="email" name="email" required>
        
        <button type="submit">Submit</button>
    </form>
</body>
</html>
"#;

    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[post("/subscribe")]
async fn subscribe(subscriber: Form<Subscriber>) -> HttpResponse {
    println!("Received subscriber: {:?}", subscriber);

    // Here you would typically save the subscriber to a database or send an email.

    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Thank you for subscribing!")
}

async fn subscribe_with_json(subscriber: Json<Subscriber>) -> HttpResponse {
    println!("Received subscriber: {:?}", subscriber);

    // Here you would typically save the subscriber to a database or send an email.

    HttpResponse::Ok()
        .content_type("application/json")
        .json(subscriber.into_inner())
}

#[derive(Default)]
struct Counters {
    to_celcius: usize,
    to_fahrenheit: usize,
}

#[derive(Default)]
struct UsageStats {
    counters: Mutex<Counters>,
}

impl UsageStats {
    fn new() -> Self {
        UsageStats::default()
    }
}

#[get("/to-celcius/{fahrenheit}")]
async fn to_celcius(fahrenheit: Path<f64>, data: Data<UsageStats>) -> HttpResponse {
    rt::spawn(async move {
        let mut counters = data.counters.lock().unwrap();
        counters.to_celcius += 1;
    });

    let celsius: f64 = (fahrenheit.into_inner() - 32.0) * 5.0 / 9.0;
    HttpResponse::Ok().json(celsius)
}

#[get("/to-fahrenheit/{celsius}")]
async fn to_fahrenheit(celsius: Path<f64>, data: Data<UsageStats>) -> HttpResponse {
    rt::spawn(async move {
        let mut counters = data.counters.lock().unwrap();
        counters.to_fahrenheit += 1;
    });

    let fahrenheit: f64 = (celsius.into_inner() * 9.0 / 5.0) + 32.0;
    HttpResponse::Ok().json(fahrenheit)
}

#[main]
async fn main() -> std::io::Result<()> {
    let usage_stats = Data::new(UsageStats::new());
    
    HttpServer::new(move || {
        App::new()
            .app_data(usage_stats.clone())
            .service(health_check)
            .service(index)
            .service(subscribe)
            .service(to_celcius)
            .service(to_fahrenheit)
            .service(
                resource("/submit")
                    .guard(guard::Header("Content-Type", "application/json"))
                    .route(web_post().to(subscribe_with_json))
            )

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}