# Tower Requirements and Usage Guide

## What is Tower?

Tower is a library of modular and reusable components for building robust networking clients and servers. It provides a middleware abstraction that lets you compose behavior for HTTP services.

## Why Tower Matters

Tower serves as the foundation for Axum (which is built on top of it) and provides several core benefits:

1. **Middleware Abstraction**: Tower standardizes the concept of "middleware" - code that sits between your application logic and the HTTP transport layer.

2. **Service Trait**: The core `Service` trait in Tower represents an asynchronous function from a request to a response, forming the basis for all middleware.

3. **Composability**: Tower components are designed to be composed together to build complex service behavior from simple parts.

4. **Reusability**: Many common concerns (timeouts, retries, rate limiting) are already packaged as reusable middleware components.

## Key Tower Concepts

### The Service Trait

The foundation of Tower is the `Service` trait:

```rust
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}
```

This trait represents an asynchronous function that:

- Takes a `Request`
- Returns a `Future` that eventually resolves to either a `Response` or an `Error`
- Has a `poll_ready` method that indicates service readiness

### Tower in Axum

Axum is built on top of Tower, with every route handler being a Tower `Service`. This means you can:

1. Add Tower middleware to Axum routes or the entire router
2. Use Tower's extensive library of middleware with Axum
3. Write your own middleware that works with Axum

## Tower Checklist for Our Project

### 1. Basic Tower Integration

- [x] **Understand Tower Service Trait**: Grasp the core concept of Tower's `Service` trait
- [x] **Understand Axum's Tower Integration**: Learn how Axum leverages Tower

### 2. Essential Middleware for API Testing

- [ ] **ServiceExt**: Use Tower's `ServiceExt` trait for testing handlers directly
  - [ ] Add the testing extension to make API tests more robust
  - [ ] Learn to use the `oneshot` method for testing
  - [ ] Create proper test helpers for API handler testing

### 3. Production-Ready Middleware

- [ ] **Timeout Middleware**: Add timeouts to prevent long-running requests
  - [ ] Configure global timeout settings
  - [ ] Add per-route timeout customization where needed
- [ ] **Rate Limiting**: Protect against abuse with rate limits
  - [ ] Implement basic IP-based rate limiting
  - [ ] Consider token bucket algorithm implementation
- [ ] **Request Tracing**: Add detailed request tracing

  - [ ] Instrument requests with tracing IDs
  - [ ] Create structured logs for requests

- [ ] **Retry Logic**: Add intelligent retry for certain operations
  - [ ] Identify which operations can be safely retried
  - [ ] Configure backoff strategies

### 4. Advanced Tower Usage

- [ ] **Custom Middleware**: Build custom middleware specific to our domain
  - [ ] Create a request validation middleware
  - [ ] Add a context-propagation middleware
- [ ] **Conditional Middleware**: Apply middleware conditionally

  - [ ] Add environment-specific middleware
  - [ ] Route-specific middleware application

- [ ] **Metrics Collection**: Use tower-http metrics
  - [ ] Track request rates and latencies
  - [ ] Integrate with monitoring systems

## Practical Tower Usage Examples

### Testing API Handlers

```rust
#[tokio::test]
async fn test_get_symbol_handler() {
    // Create a test repository
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    // Create an Axum router with our handler
    let app = Router::new()
        .route("/symbols/:id", get(handlers::get_symbol))
        .with_state(repository);

    // Get a symbol ID from the repository to test with
    let symbols = repository.list_symbols(None).await.unwrap();
    let first_symbol = symbols.first().unwrap();

    // Create a request to our handler
    let request = Request::builder()
        .uri(format!("/symbols/{}", first_symbol.id))
        .body(Body::empty())
        .unwrap();

    // Use tower::ServiceExt::oneshot to run the request through our router
    let response = app.oneshot(request).await.unwrap();

    // Assert the response is successful
    assert_eq!(response.status(), StatusCode::OK);

    // Extract the response body and verify it contains our symbol
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let symbol: Symbol = serde_json::from_slice(&body).unwrap();
    assert_eq!(symbol.id, first_symbol.id);
}
```

### Adding Timeout Middleware

```rust
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;

// In your app setup
let app = Router::new()
    .route("/symbols", get(handlers::list_symbols))
    .route("/symbols/:id", get(handlers::get_symbol))
    .layer(
        ServiceBuilder::new()
            // Add a 30-second timeout to all requests
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            .into_inner(),
    )
    .with_state(repository);
```

### Adding Rate Limiting

```rust
use tower::ServiceBuilder;
use tower_http::limit::RateLimitLayer;
use std::time::Duration;
use std::num::NonZeroU32;

// In your app setup
let app = Router::new()
    .route("/symbols", get(handlers::list_symbols))
    .layer(
        ServiceBuilder::new()
            // Allow 100 requests per minute
            .layer(RateLimitLayer::new(
                100,
                Duration::from_secs(60),
            ))
            .into_inner(),
    )
    .with_state(repository);
```

## Next Steps

1. Begin with tower for testing API handlers directly
2. Add basic middleware like timeouts and logging
3. Progress to more advanced middleware like rate limiting
4. Develop custom middleware for our specific needs
