use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub trait Repository<T> where T: Entity {

    /// A function responsible for the creation of the Repository
    fn new(config: &PersistenceConfig) -> Result<Self, String> where Self: Sized;

    /// Insert the received entity in the persistence system
    async fn create(&self, sandwich: T) -> Result<T, RepoCreateError>;

    /// Find and return one single record from the persistence system
    async fn find_one(&self, sandwich: FindSandwich) -> Result<T, RepoSelectError>;

    /// Find and return all records corresponding to the search criteria from the persistence system
    async fn find_all(&self, sandwich: FindSandwich) -> Result<Vec<T>, RepoFindAllError>;

    /// Update one single record already present in the persistence system
    async fn update(&self, sandwich: T) -> Result<T, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError>;
}