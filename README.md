# CacheN (Cache Neko Nik)

**CacheN** is a Rust-based caching library that leverages Redis for fast, in-memory caching. It provides an easy-to-use procedural macro for adding caching to your functions with support for custom cache keys and expiration times. CacheN simplifies caching logic, integrates seamlessly with your application, and helps boost performance by reducing database calls.

## ⚠️ Work in Progress

This crate is still under active development and may not be fully stable. Use with caution, and expect breaking changes in future versions.

Feel free to contribute or submit issues if you encounter bugs or want to request features!

## Features

- **Redis-backed caching**: Cache your function results in Redis.
- **Procedural macro support**: Add caching to any function with minimal boilerplate.
- **Custom cache key**: Specify the cache key for each function.
- **Custom cache expiration**: Set the expiration time for cached data.
- **Seamless integration**: Integrates easily into existing projects with minimal configuration.

## Installation

Add **CacheN** to your `Cargo.toml` dependencies:

```toml
[dependencies]
cache_n = "0.1"
```

## Usage

### Adding Cache to Functions

The core feature of **CacheN** is the `#[cache]` procedural macro. This macro can be applied to any asynchronous function that you want to cache. You can specify the cache key and expiration time directly in the attribute.


### Parameters

- `name`: (Required) The cache key (string) used for storing the result in Redis.
- `time`: (Optional) The cache expiration time in seconds (u16). Default is 60 seconds.

### How It Works

1. When you call the function, **CacheN** checks if the result is already available in Redis using the specified cache key or a default key that includes the function name and arguments (if provided).
2. If the cache is found, the cached data is returned immediately.
3. If no cache is found or if the cache has expired, the function executes as normal, fetching data from the database, and the result is cached in Redis for future use.
4. The cache expires after the specified time.

### Example Cache Interaction

**Cache Miss (Fetch from Database)**

- On the first call or after the cache expiration, the function will query the database and store the result in Redis.

**Cache Hit (Fetch from Redis)**

- On subsequent calls, the function will fetch the data from Redis instead of querying the database.


## Contributing

If you would like to contribute to **Cache-N**, feel free to fork the repository, create a feature branch, and submit a pull request. Any bug fixes, improvements, or additional features are welcome!

## License

**CacheN** is distributed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Acknowledgements

- **Redis**: The powerful in-memory data store that powers the caching functionality.
- **Serde**: For serializing and deserializing data to/from Redis.
- **SQLx**: For database integration and querying.
