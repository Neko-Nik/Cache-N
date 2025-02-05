#[cfg(test)]
mod tests {
    use cache_n::{cache_function, RedisState};
    use std::env;

    // Mock environment variables for testing
    fn setup_env() {
        env::set_var("REDIS_DB_URL", "redis://localhost:6379/0");
        env::set_var("REDIS_DB_MAX_POOL_SIZE", "5");
    }

    // Test the RedisState struct and basic operations
    #[tokio::test]
    async fn test_redis_operations() {
        setup_env();

        let redis_state = RedisState::new().await;

        // Set and get a value from the cache
        let key = "test_key";
        let value = "test_value";
        let ttl = 60;

        // Set cache value
        redis_state.set_cache(key, value, ttl).await;

        // Get cache value
        let cached_value = redis_state.get_cache(key).await;
        assert_eq!(cached_value, Some(value.to_string()));

        // Simulate cache miss
        let new_key = "non_existent_key";
        let new_value = redis_state.get_cache(new_key).await;
        assert_eq!(new_value, None);
    }

    // Test the cache_function logic
    #[tokio::test]
    async fn test_cache_function() {
        setup_env();
        // env_logger::init();

        let redis_state = RedisState::new().await;

        let key_name = "cache_func_test";
        let ttl = 10;

        // Simulate a function to be cached
        let func = || "cached_value1".to_string();

        // First call should cache the result
        let result_1 = cache_function(&redis_state, key_name, ttl, func).await;
        assert_eq!(result_1, "cached_value1");

        // Second call should return the cached result
        let result_2 = cache_function(&redis_state, key_name, ttl, func).await;
        assert_eq!(result_2, "cached_value1");

        // Check that the value is cached
        let cached_value = redis_state.get_cache(key_name).await;
        assert_eq!(cached_value, Some("cached_value1".to_string()));
    }
}
