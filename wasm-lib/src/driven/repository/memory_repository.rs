

use std::collections::HashMap;

use async_trait::async_trait;
use crate::domain::tipo_vivienda::{TipoVivienda, TipoViviendaId, Tipo};
use crate::driven::repository::{FindTipoVivienda, RepoCreateError, RepoDeleteError, RepoFindAllError, RepoSelectError, Repository, RepoUpdateError};
use crate::config::PersistenceConfig;


#[derive(Clone)]
pub struct TipoViviendaMemoryRepository {
    tipo_viviendas: HashMap<TipoViviendaId, TipoVivienda>
}



#[async_trait]
impl Repository<TipoVivienda> for TipoViviendaMemoryRepository {


    fn new() -> TipoViviendaMemoryRepository {
        TipoViviendaMemoryRepository {
            tipo_viviendas: HashMap::new(),
        }
    }


     fn create(&mut self, tipo_vivienda: TipoVivienda)  -> Result<TipoVivienda, RepoCreateError>{
        let item = tipo_vivienda.clone();
        let result = self.tipo_viviendas.insert(tipo_vivienda.id, item);
        match result {
            Some(x) => Ok(x),
            None =>  Err(RepoCreateError::Unknown("not insert".to_string()))
        } 
    }

    fn delete(&mut self, tipo_vivienda: TipoVivienda) -> Result<TipoVivienda, RepoDeleteError>{
    if self.tipo_viviendas.contains_key(&tipo_vivienda.id) {
        let result = self.tipo_viviendas.remove(&tipo_vivienda.id);
        match result {
            Some(x) => Ok(x),
            None =>  Err(RepoDeleteError::Unknown("not remove".to_string()))
        } 
    } else {
        Err(RepoDeleteError::Unknown("not contains_key".to_string()))
    }

}
}