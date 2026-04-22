mod database;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    //Loads .env file and all of its variables
    dotenv().ok();
}
