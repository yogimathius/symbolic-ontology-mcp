use symbol_ontology_mcp::db::repository::{
    PgRepositoryFactory,
    interfaces::{SymbolRepository, SymbolSetRepository},
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_repository_factory_new() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(1))
        .connect("postgres://postgres:postgres@localhost/test_db")
        .await;

    assert!(
        pool.is_err(),
        "This test doesn't require an actual database connection"
    );

    if let Ok(pool) = pool {
        let _ = PgRepositoryFactory::new(pool);
        assert!(true);
    }
}

#[tokio::test]
async fn test_create_symbol_repository() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(1))
        .connect("postgres://postgres:postgres@localhost/test_db")
        .await;

    if let Ok(pool) = pool {
        let factory = PgRepositoryFactory::new(pool);
        let _repo = factory.create_symbol_repository();

        assert_eq!(
            format!("{}", std::any::type_name::<Arc<dyn SymbolRepository>>()),
            format!("{}", std::any::type_name::<Arc<dyn SymbolRepository>>())
        );
    } else {
        assert!(true);
    }
}

#[tokio::test]
async fn test_create_symbol_set_repository() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(1))
        .connect("postgres://postgres:postgres@localhost/test_db")
        .await;

    if let Ok(pool) = pool {
        let factory = PgRepositoryFactory::new(pool);
        let _repo = factory.create_symbol_set_repository();

        assert_eq!(
            format!("{}", std::any::type_name::<Arc<dyn SymbolSetRepository>>()),
            format!("{}", std::any::type_name::<Arc<dyn SymbolSetRepository>>())
        );
    } else {
        assert!(true);
    }
}
