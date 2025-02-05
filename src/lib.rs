use r2d2::Pool;
use redis::Client as RedisClient;
use std::env;
use log::info;
use redis::Commands;


#[derive(Clone)]
pub struct RedisState {
    pub redis_pool: Pool<RedisClient>,
}

impl RedisState {
    pub async fn new() -> Self {
        let redis_url = env::var("REDIS_DB_URL").expect("REDIS_DB_URL must be set");
        let max_pool_size: u32 = env::var("REDIS_DB_MAX_POOL_SIZE")
            .unwrap_or_else(|_| "50".to_string()) // Default to 50 if not set
            .parse()
            .expect("REDIS_DB_MAX_POOL_SIZE must be a number");

        let redis_client = RedisClient::open(redis_url).unwrap();

        let pool = r2d2::Pool::builder()
            .max_size(max_pool_size)
            .build(redis_client)
            .unwrap();

        info!("Successfully connected to the Redis server");

        RedisState { redis_pool: pool }
    }

    pub async fn get_cache(&self, key: &str) -> Option<String> {
        let mut redis_conn = self.redis_pool.get().unwrap();
        // Correctly await the redis command to get the value
        let result: redis::RedisResult<Option<String>> = redis_conn.get(key);

        match result {
            Ok(value) => value,
            Err(_) => None,
        }
    }

    pub async fn set_cache(&self, key: &str, value: &str, ttl_seconds: usize) {
        let mut redis_conn = self.redis_pool.get().unwrap();
        // Convert usize to u64 for TTL
        let ttl_seconds_u64: u64 = ttl_seconds.try_into().unwrap();
        // Correctly await the redis command to set the cache value with TTL
        let _: redis::RedisResult<()> = redis_conn.set_ex(key, value, ttl_seconds_u64);
    }
}

// Caching logic
pub async fn cache_function(
    redis_state: &RedisState, 
    key_name: &str, 
    ttl: usize, 
    func: impl Fn() -> String
) -> String {
    // Check if the result is already in cache
    if let Some(cached_value) = redis_state.get_cache(key_name).await {
        println!("Cache hit for key: {} with value: {}", key_name, cached_value);
        return cached_value;
    }

    // If not in cache, call the function and cache the result
    let result = func();
    redis_state.set_cache(key_name, &result, ttl).await;
    println!("Cache miss for key: {}", key_name);
    result
}



// Sample of what the module will look like
// use cache_n::{cache, redis_initializer};

// #[cache(key_name = "func_1", duration = 10)]
// fn time_taking_function() {
//     let mut sum = 0;
//     for i in 0..100000000 {
//         sum += i;
//     }
// }

// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     redis_initializer("redis://localhost:6379/0")?;
//     time_taking_function(); // This will take time
//     time_taking_function(); // This will read from cache
// }
