use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    println!("Pinged!");
    HttpResponse::Ok().body("Hello, world!")
}

static ADDRESS: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .send_wildcard();

        let user_scope = web::scope("").service(index);

        App::new().wrap(cors).service(user_scope)
    })
    .bind(ADDRESS)?
    .run();

    let srv_handle = server.handle();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        println!("Ctrl+C received, shutting down...");
        srv_handle.stop(true).await;
    });

    println!("Running on {ADDRESS} ...");
    server.await
}
