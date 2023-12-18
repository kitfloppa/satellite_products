use std::marker::PhantomData;

use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use itertools::Itertools;
use log::info;
use tokio_postgres::Row;

use crate::persistence::repository::{HasId, Id, Repository};

use super::Client;

pub struct PostgresRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
    T: TryFrom<Row, Error = Error>,
    T: TryInto<Vec<ColumnValuePair>, Error = anyhow::Error>,
{
    marker: PhantomData<T>,
    client: Client,
    table: String,
}

impl<T> PostgresRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
    T: TryFrom<Row, Error = Error>,
    T: TryInto<Vec<ColumnValuePair>, Error = anyhow::Error>,
{
    pub fn new(client: Client, table: &str) -> Self {
        return Self {
            marker: PhantomData,
            client,
            table: String::from(table),
        };
    }
}

pub struct ColumnValuePair {
    column: String,
    value: Box<dyn tokio_postgres::types::ToSql + Sync + Send>,
}

impl ColumnValuePair {
    pub fn new<T>(column: &str, value: T) -> Self
    where
        T: tokio_postgres::types::ToSql + Sync + Send + 'static,
    {
        return Self {
            column: String::from(column),
            value: Box::new(value),
        };
    }

    pub fn column(&self) -> &str {
        return &self.column;
    }

    pub fn value(&self) -> &(dyn tokio_postgres::types::ToSql + Sync) {
        return self.value.as_ref();
    }
}

#[async_trait]
impl<T> Repository<T> for PostgresRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
    T: TryFrom<Row, Error = Error>,
    T: TryInto<Vec<ColumnValuePair>, Error = anyhow::Error>,
{
    async fn get(&self, id: Id) -> Result<Option<T>> {
        let statement = format!("SELECT * FROM {} WHERE id = $1 LIMIT 1", self.table);
        info!("statement: {}", &statement);

        let row = self
            .client
            .lock()
            .await
            .query_opt(&statement, &[&id])
            .await?;

        return Ok(match row {
            Some(row) => Some(T::try_from(row)?),
            None => None,
        });
    }

    async fn add(&mut self, entity: T) -> Result<Option<Id>> {
        // TODO: statement can be generated just one
        let column_value_pairs: Vec<ColumnValuePair> = entity.try_into()?;

        let columns = column_value_pairs.iter().map(|it| it.column()).join(", ");
        let values = (0..column_value_pairs.len())
            .map(|it| format!("${}", it + 1))
            .join(", ");

        let statement = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING id",
            self.table, columns, values
        );

        info!("statement: {}", &statement);

        let params = column_value_pairs
            .iter()
            .map(|it| it.value())
            .collect::<Vec<_>>();

        let row = self
            .client
            .lock()
            .await
            .query_one(&statement, &params)
            .await?;

        return Ok(Some(row.get(0)));
    }

    async fn delete(&mut self, id: Id) -> Result<bool> {
        let statement = format!("DELETE FROM {} WHERE id = $1", self.table);
        return Ok(self.client.lock().await.execute(&statement, &[&id]).await? != 0);
    }

    async fn update(&mut self, entity: T) -> Result<bool> {
        let id = entity.get_id().ok_or(anyhow!("entity doesn't have id"))?;

        // TODO: statement can be generated just one
        let column_value_pairs: Vec<ColumnValuePair> = entity.try_into()?;

        let columns = column_value_pairs
            .iter()
            .enumerate()
            .map(|(i, it)| format!("{} = ${}", it.column(), i + 1))
            .join(", ");

        let statement = format!(
            "UPDATE {} SET {} WHERE id = {}",
            self.table,
            columns,
            format!("${}", column_value_pairs.len() + 1)
        );

        info!("statement: {}", &statement);

        let mut params = column_value_pairs
            .iter()
            .map(|it| it.value())
            .collect::<Vec<_>>();

        params.push(&id);

        return Ok(self
            .client
            .lock()
            .await
            .execute(&statement, &params)
            .await?
            != 0);
    }

    async fn get_all(&self) -> Result<Vec<T>> {
        let statement = format!("SELECT * FROM {}", self.table);
        info!("statement: {}", &statement);

        let rows = self.client.lock().await.query(&statement, &[]).await?;

        return Ok(rows
            .into_iter()
            .map(|row| T::try_from(row))
            .collect::<Result<Vec<_>>>()?);
    }
}
