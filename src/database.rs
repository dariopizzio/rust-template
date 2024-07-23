use deadpool_diesel::postgres::{Manager, Pool};

use crate::config::Config;

pub fn get_connection_pool(config: &Config) -> Pool {
    let manager = Manager::new(
        config.database_url.clone(),
        deadpool_diesel::Runtime::Tokio1,
    );

    Pool::builder(manager)
        .build()
        .expect("There was an error building the DB pool")
}
