use bson::Document;
use mongodb::{bson::doc, options::ClientOptions, Client};
use chrono::{Utc,TimeZone};
use std::io;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

   

/* first part of project submitting api to a database*/
#[tokio::main]
    async fn main() -> mongodb::error::Result<()> { 
        let args = Args::parse();  /* Parsing a basic function */

        for _ in 0..args.count {
        println!("Hello {}!", args.name)            // Will probably replace with a match statement
        }

        
        // find().await; Call to find something in the api will probably do via app name
        // insert().await;   Call to insert a password into the api
        Ok(())
    }

    async fn find() -> mongodb::error::Result<()> {
        let client_options = ClientOptions::parse(
            "MongoDBURI",
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
            "{MongoDBURI}",
        )
        .await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("first").collection("getting-started");
        let new_doc = doc! {
            "app_name": "Parasite",
            "email/username": 2020,
            "password": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
            "hint": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
            };
    
        let insert_result = database.insert_one(new_doc.clone(), None).await;
        Ok(())
        }

        /* 
    async fn delete() -> mongodb::error::Result<()> {
        let client_options = ClientOptions::parse(
            "MongoDBURI",
        )
        .await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("first").collection("getting-started");
        // Delete all documents for movies called "Parasite":
        let delete_result = database.delete_many(
            doc! {
            "title": "Parasite"
            },
            None,
        ).await?;
        println!("Deleted {} documents", delete_result.deleted_count);

        Ok(())
        }
        
       */