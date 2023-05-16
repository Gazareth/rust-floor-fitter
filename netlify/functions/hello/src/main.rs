use lambda_runtime::{ Error };
use simple_logger::SimpleLogger;
use log::LevelFilter;

use dotenv::dotenv;

mod mongo;
mod lambda_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();    // load env vars from file
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    mongo::get_floorboards_collection("FloorBoards", "SampleFloors").await?;

    lambda_handler::start_listening().await
}
