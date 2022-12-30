use bson::{doc, oid::ObjectId };
use chrono::Utc;
use mongodb::{
    options::ClientOptions, 
    Client, Collection, Cursor
};
use std::error::Error;
use serde::{self, Serialize, Deserialize};
type ResultVoid = Result<(), Box<dyn Error>>;
type ResultBox<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct ReasonVendor {
    code: String,
    tier: i32,
    category: String,
    #[serde(rename = "isEnabled", default = "default_is_enabled")]
    is_enabled: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Reason {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    priority: String,
    name: String,
    #[serde(rename = "type")]
    reason_type: String,
    #[serde(rename = "defaultRequiredDocument")]
    default_required_document: String,
    vendor: Vec<ReasonVendor>,
    #[serde(rename = "isEnabled", default = "default_is_enabled")]
    is_enabled: bool,
    #[serde(rename = "updatedAt", with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: chrono::DateTime<Utc>,
    _class: String,
}

fn default_is_enabled() -> bool {
    return true;
}

async fn find_all_reason(client: &Client) -> ResultBox<Vec<Reason>> {
    let collection: Collection<Reason> = client
        .database("flightRefund")
        .collection("reason");
    
    let mut cursor: Cursor<Reason> = collection
        .find(None, None).await?;
    let mut reasons: Vec<Reason> = vec![];
    while cursor.advance().await? {
        match cursor.deserialize_current() {
            Ok(doc) => reasons.push(doc),
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    }
    return Ok(reasons);
}

async fn delete_all_reason(client: &Client) -> ResultBox<()> {
    let collection: Collection<Reason> = client
        .database("flightRefund")
        .collection("reason");
    collection.delete_many(doc! {}, None).await?;
    return Ok(());
}

async fn insert_all_reason(client: &Client, reasons: Vec<Reason>) -> ResultBox<()> {
    let collection: Collection<Reason> = client
        .database("flightRefund")
        .collection("reason");
    collection.insert_many(reasons, None).await?;
    return Ok(());
}

async fn list_databases(client: &Client) -> ResultVoid {
    println!("Databases");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }
    return Ok(());
}

#[tokio::main]
async fn main() -> ResultVoid {

    let uri = "mongodb://localhost:27017";
    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;
    list_databases(&client).await?;
    let reasons = find_all_reason(&client).await?;
    println!("{:#?}", reasons);
    delete_all_reason(&client).await?;
    insert_all_reason(&client, reasons).await?;
    return Ok(());
}
