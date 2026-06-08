use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger, NormalizePath};
use sqlx::PgPool;
use std::env;
use url_shortner::{redirect, shorten, cache};
 
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let redis_pool = cache::create_pool();
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
            .route("/", web::post().to(shorten))
            .route("/{short_code}",web::get().to(redirect))
            .wrap(Logger::default())  //Logs every request — method, path, status code, response time. Zero config needed.
            .wrap(NormalizePath::trim()) //ensures /shorten and /shorten/ are treated the same. Prevents subtle 404s.
            .wrap(Cors::default().allow_any_origin().allow_any_method())   //allows the frontend to send request 
    })
    
    .bind("localhost:8000")?
    .run()
    .await?;
    Ok(())
}
