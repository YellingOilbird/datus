use crate::helpers::api_errors::ApiError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, self};
use lazy_static::lazy_static;
use std::env;

type DatabasePool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub (crate) type DatabaseConnection =  r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: DatabasePool = {
        let url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(url);
        DatabasePool::new(manager).expect("Failed to initialize database")
    };
} 

pub fn connection() -> Result<DatabaseConnection, ApiError>{
    info!("Connecting to database...");
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting DB connection: {} ", e)))
}