use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
async fn health_check() -> HttpResponse { 
    HttpResponse::Ok().finish()
}
// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword // We have no .await call, so it is not needed anymore.
pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| { 
        App::new()
            .route("/health_check", web::get().to(health_check))
    }) 
    .bind("127.0.0.1:8000")? 
    .run();
    
    Ok(server)
}