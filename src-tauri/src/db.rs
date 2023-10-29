
use sqlx::PgPool;

pub async fn make_pool()-> PgPool {
    let test = "postgres://postgres:puerta756859@localhost:1402/tpoo";

    println!("Connected to database successfully");
    PgPool::connect(test).await.expect("Could not connect to database")
}

/* pub async fn _check_migrations(pool: SqlitePool) -> bool {
    let verify:Result<Option<SqliteRow>, Error> = sqlx::query("Select * from biblioteca")
        .fetch_optional(&pool)
        .await;

    verify.is_ok()
} */
