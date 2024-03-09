use mongodb::{options::ClientOptions, Client, Database};
use crate::routes::init::Result;


#[derive(Clone)]
pub struct MongoDb {
    db: Database,
}

impl MongoDb {
    pub async fn init(conection_string: String, name_database: String) -> Result<Self> {
        let mut client_options = ClientOptions
        ::parse(conection_string)
        .await?;

        client_options.app_name = Some("player-service".to_owned());
        let client = Client::with_options(client_options)?;
        let db = client.database(&name_database);
        Ok(Self { db })
    }

    pub fn get_database(&self) -> &Database {
        &self.db
    }
}

pub struct MongoDbConnectionManager {
    connection: MongoDb,
}

impl MongoDbConnectionManager {
    pub async fn new(conection_string: String, name_database: String) -> Result<Self> {
        let connection = MongoDb::init(conection_string, name_database).await?;
        Ok(Self {
            connection: connection,
        })
    }

    pub fn get_connection(&self) -> &MongoDb {
        &self.connection
    }
}