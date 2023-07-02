use async_trait::async_trait;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindTipoVivienda {
    pub id: Option<String>,
    pub calle: Option<String>,
    pub numero: i32,
    pub piso: String,
    pub codigo_postal: String,
    pub metros_cuadrados: i32,
    pub numero_aseos: i32,
    pub numero_habitaciones: i32,
    pub tipo: Tipo
}

#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoSelectError {
    NotFound,
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoFindAllError {
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoUpdateError {
    InvalidData(String),
    NotFound,
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoDeleteError {
    NotFound,
    InvalidData(String),
    Unknown(String)
}

#[async_trait]
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