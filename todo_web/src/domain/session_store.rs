use tower_sessions::{Expiry, SessionManagerLayer, cookie::time::Duration};
use tower_sessions_redis_store::{RedisStore, fred::prelude::*};
use tracing::debug;

pub async fn create_store(cache_settings: Config) -> SessionManagerLayer<RedisStore<Pool>> {
    debug!("Creating session store with settings: {:?}", cache_settings);
    let pool = Pool::new(cache_settings, None, None, None, 6).unwrap();

    pool.wait_for_connect().await.unwrap();

    let session_store = RedisStore::new(pool);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    session_layer
}
