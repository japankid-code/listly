use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};

pub type MySqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_conn(pool: &MySqlPool) -> PooledConnection<ConnectionManager<MysqlConnection>> {
    pool.get().unwrap()
}

pub fn new_pool() -> MySqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("failed to create db pool")
}
