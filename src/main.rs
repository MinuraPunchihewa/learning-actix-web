use actix_web::{ App, get, HttpResponse, HttpServer, main, post, Responder, guard };
use actix_web::web::{ Form, Json, resource, post as web_post };
use serde::{ Deserialize, Serialize };

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

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(index)
            .service(subscribe)
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