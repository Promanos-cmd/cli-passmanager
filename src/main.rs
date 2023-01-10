use bson::Document;
use mongodb::{bson::doc, options::ClientOptions, Client};
use chrono::{Utc,TimeZone};
use clap::Parser;

/* first part of project submitting api to a database*/
#[tokio::main]
    async fn main() -> mongodb::error::Result<()> {
       find().await;
       Ok(())
    }

    async fn find() -> mongodb::error::Result<()> {
        let client_options = ClientOptions::parse(
            "mongodb+srv://paul-test:Pr12rankis@serverlessinstance0.zwoeb.mongodb.net/?retryWrites=true&w=majority",
        )
        .await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("first").collection("getting-started");

        let find : Document = database
        .find_one(
                doc! {
                        "title": "Parasite"
                },
                None,
            ).await?
            .expect("Missing 'Parasite' document.");
            println!("Movie: {}", find);
            Ok(())
        }

    async fn insert() -> mongodb::error::Result<()> {
        let client_options = ClientOptions::parse(
            "mongodb+srv://paul-test:Pr12rankis@serverlessinstance0.zwoeb.mongodb.net/?retryWrites=true&w=majority",
        )
        .await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("first").collection("getting-started");
        let new_doc = doc! {
            "title": "Parasite",
            "year": 2020,
            "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
            "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
            };
    
        let insert_result = database.insert_one(new_doc.clone(), None).await;
        Ok(())
        }
