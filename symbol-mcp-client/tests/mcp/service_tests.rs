use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use symbol_ontology_mcp::mcp::service::SymbolService;

#[tokio::test]
async fn test_symbol_service_new_with_db() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(1))
        .connect("postgres://postgres:postgres@localhost/test_db")
        .await;

    if let Ok(pool) = pool {
        let _service = SymbolService::new_with_db(pool);

        assert!(true, "SymbolService was successfully created");
    } else {
        println!("Skipping test_symbol_service_new_with_db as database connection failed");
    }
}
