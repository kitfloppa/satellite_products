use std::sync::Arc;

use tokio::sync::RwLock;

use self::repository::HasId;

pub mod model;
pub mod repository;

pub type Repository<T> = Arc<RwLock<dyn self::repository::Repository<T> + Send + Sync>>;
pub type InMemoryRepository<T> = Arc<RwLock<self::repository::InMemoryRepository<T>>>;

pub fn create_inmemory_repository<T>() -> InMemoryRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
{
    return Arc::new(tokio::sync::RwLock::new(
        self::repository::InMemoryRepository::<T>::new(),
    ));
}
