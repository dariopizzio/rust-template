use std::env;

use deadpool_diesel::postgres::{Manager, Pool};

pub fn get_connection_pool() -> Pool {
    // TODO create config struct with dotenvy
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager).build().unwrap()
}
