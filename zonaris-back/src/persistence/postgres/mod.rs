use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};
use tokio_postgres::Row;

use crate::persistence::repository::HasId;

use self::repository::ColumnValuePair;

pub mod repository;

pub type Client = Arc<Mutex<tokio_postgres::Client>>;
pub type PostgresRepository<T> = Arc<RwLock<self::repository::PostgresRepository<T>>>;

pub fn create_postgres_repository<T>(client: Client, table: &str) -> PostgresRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
    T: TryFrom<Row, Error = anyhow::Error>,
    T: TryInto<Vec<ColumnValuePair>, Error = anyhow::Error>,
{
    return Arc::new(tokio::sync::RwLock::new(
        self::repository::PostgresRepository::<T>::new(client, table),
    ));
}
