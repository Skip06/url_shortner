use actix_web::{web, HttpResponse};
use base62::encode;
use serde::Deserialize;
use sqlx::{PgPool, Row};
use deadpool_redis::Pool as RedisPool;

pub mod cache;


#[derive(Deserialize)]
pub struct ShortenRequest {
   pub url: String,
}

pub async fn shorten(
    body: web::Json<ShortenRequest>,
    pool: web::Data<PgPool>
    ) -> Result<HttpResponse, actix_web::Error> {

    let url = &body.url;
    let row = sqlx::query(
        "insert into urls (short_code, original_url) values ('', $1) returning id"
    )
    .bind(url) 
    .fetch_one(pool.get_ref()) //gives Result of one row from the db
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let id: i64 = row.get("id");
    let short_code = encode(id as u128);

    sqlx::query("update urls set short_code = $1 where id = $2")
        .bind(&short_code) // insertin the $1 value 
        .bind(id)
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
    pool: web::Data<PgPool>,
    cache_pool: web::Data<RedisPool>
) -> Result<HttpResponse, actix_web::Error> {
    let short_code = path.into_inner();

    
    // 1. Try to get from cache
    if let Some(original_url) = cache::get_cached_url(cache_pool.get_ref(), &short_code).await {
        println!("cache hit: {} -> {}", short_code, original_url);
        return Ok(HttpResponse::TemporaryRedirect()
            .insert_header(("Location", original_url))
            .finish());
    }

    // 2. If not in cache, get from database
    let row = sqlx::query("select original_url from urls where short_code = $1")
        .bind(&short_code)
        .fetch_one(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let original_url: String = row.get("original_url");

    // 3. Save to cache for future requests
    cache::set_cached_url(cache_pool.get_ref(), &short_code, &original_url).await;
    println!("cache miss: {} -> {}. saved to cache.", short_code, original_url);

    Ok(HttpResponse::TemporaryRedirect()
        .insert_header(("Location", original_url))
        .finish())
}
