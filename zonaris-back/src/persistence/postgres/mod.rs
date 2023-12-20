use std::{ops::Deref, sync::Arc};

use tokio::sync::{Mutex, RwLock};
use tokio_postgres::{
    types::{FromSql, IsNull, ToSql},
    Row,
};

use crate::persistence::repository::HasId;

use self::repository::ColumnValuePair;

use super::repository::{Id, Reference};

pub mod migration;
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

#[cfg(feature = "postgres")]
impl<T> std::fmt::Debug for Reference<T>
where
    T: HasId,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reference").field("id", &self).finish()
    }
}

#[cfg(feature = "postgres")]
impl<'a, T> FromSql<'a> for Reference<T>
where
    T: HasId,
{
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let id = Id::from_sql(ty, raw)?;
        return Ok(Reference::new(id));
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        return <Id as FromSql>::accepts(ty);
    }
}

#[cfg(feature = "postgres")]
impl<T> ToSql for Reference<T>
where
    T: HasId,
{
    fn to_sql(
        &self,
        ty: &tokio_postgres::types::Type,
        out: &mut tokio_util::bytes::BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        return self.deref().to_sql(ty, out);
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool
    where
        Self: Sized,
    {
        return <Id as ToSql>::accepts(ty);
    }

    fn to_sql_checked(
        &self,
        ty: &tokio_postgres::types::Type,
        out: &mut tokio_util::bytes::BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        return self.deref().to_sql_checked(ty, out);
    }
}
