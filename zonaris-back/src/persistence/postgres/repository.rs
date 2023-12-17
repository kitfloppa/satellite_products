use std::marker::PhantomData;

use anyhow::{Error, Result};
use async_trait::async_trait;
use tokio_postgres::Row;

use crate::persistence::repository::{HasId, Id, Repository};

pub struct PostgresRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
    T: TryFrom<Row, Error = Error>,
{
    marker: PhantomData<T>,
    client: tokio_postgres::Client,
    table: String,
}

impl<T> PostgresRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
    T: TryFrom<Row, Error = Error>,
{
    pub fn new(client: tokio_postgres::Client, table: String) -> Self {
        return Self {
            marker: PhantomData,
            client,
            table,
        };
    }
}

#[async_trait]
impl<T> Repository<T> for PostgresRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
    T: TryFrom<Row, Error = Error>,
{
    async fn get(&self, id: Id) -> Result<Option<T>> {
        let statement = format!("SELECT * FROM {} WHERE id = $1 LIMIT 1", self.table);
        let row = self.client.query_opt(&statement, &[&id]).await?;

        return Ok(match row {
            Some(row) => Some(T::try_from(row)?),
            None => None,
        });
    }

    async fn add(&mut self, mut entity: T) -> bool {
        todo!();
    }

    async fn delete(&mut self, id: Id) -> Result<bool> {
        let statement = format!("DELETE FROM {} WHERE id = $1", self.table);
        return Ok(self.client.execute(&statement, &[&id]).await? != 0);
    }

    async fn update(&mut self, entity: T) -> bool {
        todo!();
    }

    async fn get_all(&self) -> Vec<T> {
        let statement = format!("SELECT * FROM {}", self.table);
        let rows = self.client.query(&statement, &[]).await.unwrap(); // TODO: delete unwrap

        return rows
            .into_iter()
            .map(|row| T::try_from(row))
            .collect::<Result<Vec<_>>>()
            .unwrap(); // TODO: delete unwrap
    }
}
