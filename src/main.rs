use bson::Document;
use mongodb::{bson::doc, options::ClientOptions, Client};
use chrono::{Utc,TimeZone};
use clap::{Parser, Subcommand, Command, Args};





#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Variables {
   /// Input Subcommand
   #[clap(subcommand)]
   pub input: Input,
}

/* If more sub commands are to be written it will be in this section */
#[derive(Debug,Subcommand)]
pub enum Input {
    /// First subcommand to create the json to create a password
    Create(AppName)
}
   
#[derive(Debug, Args)]
pub struct AppName {
    ///Users Username
    pub username: String,
    ///Users Password
    pub password: String,
    /// The name of the account Connected App
    pub app: String,
}




/* first part of project submitting api to a database

This will be the data part of the project 

Thinking of porting this to diesel and a postgresql database to not run the cost of mongoDB usage



*/
#[tokio::main]
    async fn main() -> mongodb::error::Result<()> { 
        let args = Variables::parse(); 

        println!("{:?}", args );

        
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
            "MongoDBURI",
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
