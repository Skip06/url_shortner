use actix_web::{body, web, App, HttpRequest, HttpResponse, HttpServer};
use base62::encode;
use serde::Deserialize;
use sqlx::PgPool;
use serde_json;
use std::env; //env::var("var") it gets the value of the variable in the .env file
use dotenv::dotenv; // loads the .env file
use url_shortner::{redirect, shorten};

 
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::post().to(shorten))
            .route("/{short_code}",web::get().to(redirect))
    })
    .bind("localhost:8000")?
    .run()
    .await?;
    Ok(())
}
