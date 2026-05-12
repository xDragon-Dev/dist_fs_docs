mod calls;
mod checksum;

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pass = common::auth::hash_password("12345678".as_bytes()).unwrap();
    println!("{pass}");
    Ok(())
}
