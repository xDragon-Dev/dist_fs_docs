mod calls;
mod checksum;

use common::auth::generate_jwt;
use common::types::jwt_types::{self, *};

use dotenvy::dotenv;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pass = common::auth::hash_password("12345678".as_bytes()).unwrap();
    println!("{pass}");

    let token_claims: TokenClaims = TokenClaims {
        sub: "Juanito".into(),
        user_role: jwt_types::Role::Admin,
        exp: i64::MAX,
        iat: i64::MIN,
    };
    let jwt = generate_jwt(token_claims).unwrap();

    println!("{jwt}");
    (0..10).into_iter().for_each(|_| {
        println!("{}", Uuid::new_v4());
    });
    Ok(())
}
