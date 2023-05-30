use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};

pub type MySqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

// use crate::MIGRATIONS;
// use diesel_migrations::{EmbeddedMigrations, MigrationHarness};

pub fn new_pool() -> MySqlPool {
    dotenv().ok().expect("no .env file found");

    let database_url = env::var("DATABASE_URL_MYSQL").expect("DATABASE_URL_MYSQL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("failed to create db pool")
}

// fn establish_connection() -> MysqlConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     return MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url));
// }

// pub fn run_migrations() {
//     let db_conn = &mut establish_connection();

//     db_conn.run_pending_migrations(MIGRATIONS).unwrap();
// }

// fn run_migrations2(
//     connection: &mut impl MigrationHarness<MysqlConnection>,
// ) -> Result<Vec<MigrationVersion>> {
//     connection
//         .run_pending_migrations(MIGRATIONS)
//         .expect("migrations failed to run");
// }
