static DB: std::sync::OnceLock<sqlx::SqlitePool> = std::sync::OnceLock::new();

pub async fn init_db() {
    // Get db url form env
    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./mydata.db".to_string());
    let pool = sqlx::SqlitePool::connect(&db_url)
        .await
        .expect("Could not make DB pool.");
    //sqlx::migrate!().run(&pool).await.expect("Could not run DB migrations.");
    let _ = DB.set(pool);
}

pub fn get_db<'a>() -> &'a sqlx::SqlitePool {
    DB.get().expect("database unitialized")
}

