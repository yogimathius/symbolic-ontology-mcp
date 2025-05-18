use sqlx::{Pool, sqlite::Sqlite, sqlite::SqlitePool};

#[derive(Debug, sqlx::FromRow)]
struct TestSymbol {
    id: String,
    name: String,
    category: String,
    description: String,
}

async fn create_test_pool() -> Pool<Sqlite> {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS symbols (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            description TEXT NOT NULL,
            interpretations TEXT DEFAULT '{}',
            related_symbols TEXT DEFAULT '[]',
            properties TEXT DEFAULT '{}'
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let test_symbols = vec![
        sqlx::query(
            r#"
            INSERT INTO symbols (id, name, category, description, interpretations, related_symbols, properties)
            VALUES ('sun', 'Sun', 'nature', 'Celestial body at the center of our solar system', 
                    '{"default":"Represents life, energy, and vitality"}',
                    '["light", "day"]',
                    '{"element":"fire"}')
            "#,
        ),
        sqlx::query(
            r#"
            INSERT INTO symbols (id, name, category, description, interpretations, related_symbols, properties) 
            VALUES ('moon', 'Moon', 'nature', 'Natural satellite of Earth',
                    '{"default":"Represents intuition, femininity and cycles"}',
                    '["night", "tide"]',
                    '{"element":"water"}')
            "#,
        ),
        sqlx::query(
            r#"
            INSERT INTO symbols (id, name, category, description, interpretations, related_symbols, properties)
            VALUES ('light', 'Light', 'concept', 'Electromagnetic radiation visible to the human eye',
                    '{"default":"Represents knowledge, truth, and enlightenment"}',
                    '["sun", "illumination"]',
                    '{"element":"fire"}')
            "#,
        ),
    ];

    for query in test_symbols {
        query.execute(&pool).await.unwrap();
    }

    pool
}

#[tokio::test]
async fn test_list_symbols_integration() {
    let pool = create_test_pool().await;

    let symbols = sqlx::query_as::<_, TestSymbol>("SELECT * FROM symbols")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert!(!symbols.is_empty());
}

#[tokio::test]
async fn test_get_symbol_integration() {
    let pool = create_test_pool().await;

    let symbol = sqlx::query_as::<_, TestSymbol>("SELECT * FROM symbols WHERE id = 'sun'")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(symbol.id, "sun");
    assert_eq!(symbol.name, "Sun");
    assert_eq!(symbol.category, "nature");
}

#[tokio::test]
async fn test_search_symbols_integration() {
    let pool = create_test_pool().await;

    let symbols = sqlx::query_as::<_, TestSymbol>(
        "SELECT * FROM symbols WHERE name LIKE '%Light%' OR description LIKE '%Light%'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(!symbols.is_empty());
    assert!(symbols.iter().any(|s| s.id == "light"));
}

#[tokio::test]
async fn test_error_handling_symbol_not_found() {
    let pool = create_test_pool().await;

    let result =
        sqlx::query_as::<_, TestSymbol>("SELECT * FROM symbols WHERE id = 'nonexistent-id'")
            .fetch_one(&pool)
            .await;

    assert!(result.is_err());
    match result {
        Err(err) => {
            let error_string = err.to_string();
            assert!(error_string.contains("no rows returned"));
        }
        _ => panic!("Expected error response"),
    }
}
