use deadpool_redis::{Config, Pool, Runtime};
use deadpool_redis::redis::AsyncCommands;

pub fn create_pool() -> Pool {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let cfg = Config::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1)).expect("failed to create pool")
}

pub async fn get_cached_url(pool: &Pool, short_code: &str) -> Option<String> {
    let mut conn = pool.get().await.ok()?;
    conn.get(short_code).await.ok()
}

pub async fn set_cached_url(pool: &Pool, short_code: &str, original_url: &str) {
    if let Ok(mut conn) = pool.get().await {
        let _: Result<(), _> = conn.set_ex(short_code, original_url, 3600).await;
    }
}