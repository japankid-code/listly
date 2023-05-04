use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};

pub type MySqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn new_pool() -> MySqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL_MYSQL").expect("DATABASE_URL_MYSQL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("failed to create db pool")
}
