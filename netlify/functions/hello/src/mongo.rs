use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, Database};
use std::env;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Floorboard {
    label: String,
    length: u32,
}

pub async fn get_floorboards_collection(db_name:&str, collection_name:&str)  -> mongodb::error::Result<()> {
    let db = get_database(db_name).await?;

    // Get a handle to a collection of `Floorboard`.
    let typed_collection = db.collection::<Floorboard>(collection_name);

    let mut cursor = typed_collection.find(None, None).await?;

    // Iterate over the results of the cursor.
    while let Some(floorboard) = cursor.try_next().await? {
        println!("label: {}", floorboard.label);
    }

    Ok(())
}

pub async fn get_database(db_name:&str) -> mongodb::error::Result<Database> {
    let uri = env::var("MONGODB_CONNECTION_URI").expect("MONGODB_CONNECTION_URI must be set.");
    let mut client_options =
        ClientOptions::parse(uri)
            .await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    // Get a handle to a database.
    Ok(client.database(db_name))
}
