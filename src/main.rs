use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};



pub async fn shorten(_req: HttpRequest) -> HttpResponse {
    print!("inside handler function ");
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
     let _ = HttpServer::new(||{
         App::new()
             .route("/", web::post().to(shorten))
         
     })
    .bind("localhost:8000")?
    .run().await;
     Ok(())

}
