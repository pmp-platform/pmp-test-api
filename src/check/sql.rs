use crate::models::{SqlCheckResult, SqlConfig};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing::{debug, error, info, instrument};

/// Check a SQL database connection and retrieve information
#[instrument(skip(config), fields(identifier = %config.identifier, driver = %config.driver))]
pub async fn check_sql(config: SqlConfig) -> SqlCheckResult {
    info!("Checking SQL database: {}", config.identifier);

    match config.driver.as_str() {
        "postgres" => check_postgres(config).await,
        "mysql" => check_mysql(config).await,
        driver => {
            error!("Unsupported SQL driver: {}", driver);
            SqlCheckResult {
                success: false,
                driver: driver.to_string(),
                host: config.host,
                port: config.port,
                database: config.database,
                tables: None,
                error: Some(format!("Unsupported SQL driver: {}", driver)),
            }
        }
    }
}

/// Check a PostgreSQL database connection
#[instrument(skip(config))]
async fn check_postgres(config: SqlConfig) -> SqlCheckResult {
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.database
    );

    debug!("Attempting to connect to PostgreSQL database");

    // Create a connection pool with timeout
    let pool_result = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&connection_string)
        .await;

    match pool_result {
        Ok(pool) => {
            debug!("Successfully connected to PostgreSQL");

            // Try to list tables
            let tables_result = sqlx::query_as::<_, (String,)>(
                "SELECT table_name
                 FROM information_schema.tables
                 WHERE table_schema = 'public'
                 ORDER BY table_name"
            )
            .fetch_all(&pool)
            .await;

            match tables_result {
                Ok(rows) => {
                    let tables: Vec<String> = rows.into_iter().map(|(name,)| name).collect();
                    info!("Successfully retrieved {} tables", tables.len());

                    SqlCheckResult {
                        success: true,
                        driver: "postgres".to_string(),
                        host: config.host,
                        port: config.port,
                        database: config.database,
                        tables: Some(tables),
                        error: None,
                    }
                }
                Err(e) => {
                    error!("Failed to retrieve tables: {}", e);
                    SqlCheckResult {
                        success: false,
                        driver: "postgres".to_string(),
                        host: config.host,
                        port: config.port,
                        database: config.database,
                        tables: None,
                        error: Some(format!("Failed to retrieve tables: {}", e)),
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to connect to PostgreSQL: {}", e);
            SqlCheckResult {
                success: false,
                driver: "postgres".to_string(),
                host: config.host,
                port: config.port,
                database: config.database,
                tables: None,
                error: Some(format!("Connection failed: {}", e)),
            }
        }
    }
}

/// Check a MySQL database connection
#[instrument(skip(config))]
async fn check_mysql(config: SqlConfig) -> SqlCheckResult {
    let connection_string = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.database
    );

    debug!("Attempting to connect to MySQL database");

    // Create a connection pool with timeout
    let pool_result = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&connection_string)
        .await;

    match pool_result {
        Ok(pool) => {
            debug!("Successfully connected to MySQL");

            // Try to list tables
            let tables_result = sqlx::query_as::<_, (String,)>(
                "SELECT table_name
                 FROM information_schema.tables
                 WHERE table_schema = ?
                 ORDER BY table_name"
            )
            .bind(&config.database)
            .fetch_all(&pool)
            .await;

            match tables_result {
                Ok(rows) => {
                    let tables: Vec<String> = rows.into_iter().map(|(name,)| name).collect();
                    info!("Successfully retrieved {} tables", tables.len());

                    SqlCheckResult {
                        success: true,
                        driver: "mysql".to_string(),
                        host: config.host,
                        port: config.port,
                        database: config.database,
                        tables: Some(tables),
                        error: None,
                    }
                }
                Err(e) => {
                    error!("Failed to retrieve tables: {}", e);
                    SqlCheckResult {
                        success: false,
                        driver: "mysql".to_string(),
                        host: config.host,
                        port: config.port,
                        database: config.database,
                        tables: None,
                        error: Some(format!("Failed to retrieve tables: {}", e)),
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to connect to MySQL: {}", e);
            SqlCheckResult {
                success: false,
                driver: "mysql".to_string(),
                host: config.host,
                port: config.port,
                database: config.database,
                tables: None,
                error: Some(format!("Connection failed: {}", e)),
            }
        }
    }
}
