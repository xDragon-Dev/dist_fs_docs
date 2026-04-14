mod auth;
mod validation;

use auth::*;
use chrono::{Duration, prelude::*};
use uuid::Uuid;

use sqlx::postgres::PgPoolOptions;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    //Loads .env file and all of its variables
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("Enviroment variable \"DATABASE_URL\" must be set");

    let pool = PgPoolOptions::new()
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
    let _obtained_uuid = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
    let _exp_time = Utc::now()
        .checked_add_signed(Duration::days(30))
        .expect("Date out of range")
        .timestamp();

    let jwt_payload = JWTClaims::default();

    let json_string = serde_json::to_string_pretty(&jwt_payload).unwrap();
    println!("{}", json_string);

    let jwt_payload_from_json = serde_json::from_str::<JWTClaims>(&json_string).unwrap();
    println!("{:#?}", jwt_payload_from_json);

    let generated_jwt = generate_jwt(jwt_payload_from_json).unwrap();
    println!("{}", generated_jwt);

    for _i in 0..6 {
        println!("{}", Uuid::new_v4());
    }
}
