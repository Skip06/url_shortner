use actix_web::{body, web, App, HttpRequest, HttpResponse, HttpServer};
use base62::encode;
use serde::Deserialize;
use sqlx::PgPool;
use serde_json;
use std::env; //env::var("var") it gets the value of the variable in the .env file
use dotenv::dotenv; // loads the .env file


//we will send the post req body as a json obj
#[derive(Deserialize)]
pub struct ShortenRequest {
   pub url: String,
}

pub async fn shorten(
    body: web::Json<ShortenRequest>,
    pool: web::Data<PgPool> 
    ) -> Result<HttpResponse, actix_web::Error> {

    let url = &body.url;
    let row = sqlx::query!(
        "insert into urls (short_code, original_url) values ('', $1) returning id ", url
        ).fetch_one(pool.get_ref()).await.map_err(actix_web::error::ErrorInternalServerError)?;

    let short_code = encode(row.id as u128);

    let result= sqlx::query!("update urls set short_code = $1 where id = $2", short_code, row.id)
        .execute(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("shortened: {} -> {}", url, short_code);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "short_code": short_code,
        "original_url": url,
    })))
}

pub async fn redirect(
    path: web::Path<String>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, actix_web::Error> {
    let short_code = path.into_inner();
    let row = sqlx::query!("select original_url from urls where short_code = $1", short_code)
        .fetch_one(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::TemporaryRedirect().insert_header(("Location", row.original_url)).finish()) 
    //.insert_header(("Location", row.original_url)): For a redirect to work, the HTTP response must include a Location header telling the browser where to go. This injects the original URL we grabbed from the database.
    //A 307 (or 302) is ideal for URL shorteners because it tells the browser: "Go to this new URL for now, but don't cache this choice forever. Next time, ask my server again." This lets you track analytics on every click.
}  
 
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
