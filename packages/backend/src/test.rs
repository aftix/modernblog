use diesel::RunQueryDsl;
use rocket::local::asynchronous::Client;
use super::{rocket, SQLite};
use diesel_migrations::{embed_migrations, EmbedMigrations};

embed_migrations!();


// Get a rocket instance for tests with mock db
#[doc(hidden)]
pub async fn setup() -> Client {
    use crate::schema::posts::dsl::*;
    use crate::schema::images::dsl::*;

    let client = Client::tracked(rocket()).await.expect("Valid rocket instance");

    {
        let sql = SQLite::get_one(client.rocket()).await.expect("Couldn't get a connection");
        sql.run(|c| {
            diesel::sql_query("PRAGMA foreign_keys = ON").execute(c).expect("Foregin Keys Failed");
            embedded_migrations::run(c).expect("Failed to run migrations");
        }).await;
        // TODO: Populate fake database
    }

    client
}