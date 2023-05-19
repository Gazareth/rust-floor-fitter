use lambda_runtime::{ Error };
use simple_logger::SimpleLogger;
use log::LevelFilter;

use dotenv::dotenv;

mod lambda_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();    // load env vars from file
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    println!("Starting to listen...");
    lambda_handler::start_listening().await
}
