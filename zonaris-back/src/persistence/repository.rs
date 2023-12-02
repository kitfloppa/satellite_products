use std::collections::HashMap;

use async_trait::async_trait;

pub type Id = i32;

pub trait HasId {
    fn get_id(&self) -> Option<Id>;
    fn set_id(&mut self, id: Id);
}

// TODO: replace bool-s with Result<(), RepositoryError>
#[async_trait]
pub trait Repository<T>
where
    T: HasId,
{
    /// Some(T) if record with given id found else None
    async fn get(&self, id: Id) -> Option<T>;

    /// true if successfully added or if it's impossible to determine status of operation else false (for example in case when entity already with same key already in repository)
    async fn add(&mut self, entity: T) -> bool;

    /// true if successfully deleted or if it's impossible to determine status of operation else false
    async fn delete(&mut self, id: Id) -> bool;

    /// true if successfully updated or if it's impossible to determine status of operation else false (for example in case when `entity.get_id().is_none()`)
    async fn update(&mut self, entity: T) -> bool;

    // TODO: this function can have performance issue. recomended implementation with pagination (offset, size)
    async fn get_all(&self) -> Vec<T>;
}

pub struct InMemoryRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync, // TODO: i don't sure why
{
    data: HashMap<Id, T>,
    next_id: Id,
}

impl<T> InMemoryRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
{
    pub fn new() -> InMemoryRepository<T> {
        return InMemoryRepository::<T> {
            data: HashMap::new(),
            next_id: 0,
        };
    }

    fn get_unoccupied_id(&mut self) -> Id {
        while self.data.contains_key(&self.next_id) {
            self.next_id += 1;
        }

        return self.next_id;
    }
}

impl<T> From<&[T]> for InMemoryRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
{
    fn from(elements: &[T]) -> Self {
        let mut repository = InMemoryRepository::<T>::new();
        let mut id: Id = 0;
        for element in elements {
            let mut element = element.clone();
            element.set_id(id);
            repository.data.insert(id, element);
            id += 1;
        }

        repository.next_id = id;

        return repository;
    }
}

impl<T> From<Vec<T>> for InMemoryRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
{
    fn from(elements: Vec<T>) -> Self {
        let mut repository = InMemoryRepository::<T>::new();
        let mut id: Id = 0;
        for mut element in elements {
            element.set_id(id);
            repository.data.insert(id, element);
            id += 1;
        }

        repository.next_id = id;

        return repository;
    }
}

#[async_trait]
impl<T> Repository<T> for InMemoryRepository<T>
where
    T: HasId,
    T: Clone,
    T: Send + Sync,
{
    async fn get(&self, id: Id) -> Option<T> {
        return self.data.get(&id).cloned();
    }

    async fn add(&mut self, mut entity: T) -> bool {
        let key = if let Some(id) = entity.get_id() {
            id
        } else {
            self.get_unoccupied_id()
        };

        if self.data.contains_key(&key) {
            return false;
        }

        entity.set_id(key);
        self.data.insert(key, entity);
        return true;
    }

    async fn delete(&mut self, id: Id) -> bool {
        return self.data.remove(&id).is_some();
    }

    async fn update(&mut self, entity: T) -> bool {
        let key = if let Some(id) = entity.get_id() {
            id
        } else {
            return false;
        };

        if let Some(data) = self.data.get_mut(&key) {
            *data = entity;
        }

        return true;
    }

    async fn get_all(&self) -> Vec<T> {
        return self.data.values().cloned().collect();
    }
}