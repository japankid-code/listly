use crate::diesel_migrations::MigrationHarness;
use crate::MIGRATIONS;
use crate::{MySqlPool, MySqlPooledConnection};
use actix_web::web;

pub fn run_all_migrations(pool: web::Data<MySqlPool>) {
    let conn: &mut MySqlPooledConnection = &mut pool.get().unwrap();
    conn.run_pending_migrations(MIGRATIONS)
        .expect("migrations failed");
}
