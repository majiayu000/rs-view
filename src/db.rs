use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
// use diesel::r2d2::ConnectionManager;
use diesel_migrations::{
    EmbeddedMigrations,
    MigrationHarness};
// use diesel::r2d2::Pool;
use lazy_static::lazy_static;
use r2d2;
use std::env;

// type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// lazy_static! {
//     static ref POOL: Pool = {
//         let db_url = env::var("DATABASE_URL").expect("Database url not set!!");
//         let manager = ConnectionManager::<PgConnection>::new(db_url);
//         Pool::new(manager).expect("Failed to create db pool")
//     };
// }

// pub fn init() {
//     lazy_static::initialize(&POOL);
//     let conn = connection().expect("Failed to get db connection");
//     conn.run_pending_migrations(MIGRATIONS).unwrap();
//     // embedded_migrations::run(&conn).unwrap();
// }

// pub fn connection() -> Result<DbConnection, error> {
//     POOL.get()
//         .map_err(|e| error::new(500, format!("Failed getting db connection: {e}")))
// }

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic("Error connecting to {}", database_url))
}