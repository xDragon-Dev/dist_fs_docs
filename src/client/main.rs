mod calls;
mod checksum;

use common::auth::generate_jwt;
use common::types::jwt_types::*;

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token_claims: TokenClaims = TokenClaims {
        sub: "Juanito".into(),
        user_role: TokenRole::Admin,
        exp: i64::MAX,
    };
    let jwt = generate_jwt(token_claims).unwrap();

    println!("{jwt}");
    println!("{}", uuid::Uuid::new_v4());
    Ok(())
}
