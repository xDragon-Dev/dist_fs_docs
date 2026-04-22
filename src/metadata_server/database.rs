use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;

async fn _database_examples() {
    let database_url =
        env::var("DATABASE_URL").expect("Enviroment variable \"DATABASE_URL\" must be set");

    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    /*

    sqlx::query(
        "INSERT INTO users(usr_name, usr_pass) VALUES ('Juanito', '1234'), ('Pedrito', '5678')",
    )
    .execute(&pool)
    .await
    .unwrap();

    let retults: Vec<User> = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:#?}", retults);

    */

    let _uuid = Uuid::new_v4();
    let _str_uuid = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
}
