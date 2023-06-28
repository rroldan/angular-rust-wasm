
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Tipo {
    Apartamento,
    Casa,
    Chalet
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipoViviendaId(Option<String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipoViviendaCalle(Option<String>);



#[derive(Debug, Deserialize,Serialize,Clone)]
pub struct TipoVivienda {
    pub id: TipoViviendaId,
    pub calle: TipoViviendaCalle,
    pub numero: i32,
    pub piso: String,
    pub codigo_postal: String,
    pub metros_cuadrados: i32,
    pub numero_aseos: i32,
    pub numero_habitaciones: i32,
    pub tipo: Tipo
}

impl TipoViviendaId {
    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}

impl TryFrom<String> for TipoViviendaId {
    type Error = &'static str;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        if id.is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(id)))
        }
    }
}

impl TipoViviendaCalle {
    pub fn value(&self) -> &Option<String> {
        &self.0
    }
}

impl TryFrom<String> for TipoViviendaCalle {
    type Error = &'static str;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        if id.is_empty() {
            Err("Calle no puede estar vacia")
        } else {
            Ok(Self(Some(id)))
        }
    }
}
impl TipoVivienda {
    pub fn new(id: String, calle: String, numero:i32, piso:String, codigo_postal:String, metros_cuadrados:i32, numero_aseos:i32,numero_habitaciones:i32,tipo:Tipo) -> Result<TipoVivienda, String> {
        let tipo_vivienda_id = TipoViviendaId::try_from(id)?;
        let tipo_vivienda_calle = TipoViviendaCalle::try_from(calle)?;
        
        
        Ok(Self {
            id:tipo_vivienda_id, 
            calle:tipo_vivienda_calle, numero, piso, codigo_postal, metros_cuadrados, numero_aseos,numero_habitaciones,tipo
        })
    }

    pub fn id(&self) -> &TipoViviendaId {
        &self.id
    }

    pub fn calle(&self) -> &TipoViviendaCalle {
        &self.calle
    }


}

mod tests {
    use super::*;

    const TIPO_VIVIENDA_ID: &str = "tipo-vivienda-id";
    const TIPO_VIVIENDA_CALLE: &str = "San Isidro";
    const TIPO_VIVIENDA_NUMERO: i32 = 4;
    const TIPO_VIVIENDA_PISO: &str = "1C";
    const TIPO_VIVIENDA_CODIGO_POSTAL: &str = "28350";
    const TIPO_VIVIENDA_METROS_CUADRADOS: i32 = 80;
    const TIPO_VIVIENDA_NUMERO_ASEOS: i32 = 1;
    const TIPO_VIVIENDA_NUMERO_HABITACIONES: i32 = 2;
    const TIPO_VIVIENDA_TIPO: Tipo = Tipo::Apartamento;

     #[test]
    fn should_create_the_expected_tipo_vivienda() {
        let tipo_vivienda = TipoVivienda::new ( 
            TIPO_VIVIENDA_ID.to_string(),
            TIPO_VIVIENDA_CALLE.to_string(),
            TIPO_VIVIENDA_NUMERO,
            TIPO_VIVIENDA_PISO.to_string(),
            TIPO_VIVIENDA_CODIGO_POSTAL.to_string(),
            TIPO_VIVIENDA_METROS_CUADRADOS,
            TIPO_VIVIENDA_NUMERO_ASEOS,
            TIPO_VIVIENDA_NUMERO_HABITACIONES,
            TIPO_VIVIENDA_TIPO).unwrap();

        assert_eq!(tipo_vivienda.id().value().as_ref().unwrap(), TIPO_VIVIENDA_ID);
        assert_eq!(tipo_vivienda.calle().value().as_ref().unwrap(), TIPO_VIVIENDA_CALLE);
        assert_eq!(tipo_vivienda.numero, TIPO_VIVIENDA_NUMERO);
        assert_eq!(tipo_vivienda.piso, TIPO_VIVIENDA_PISO);
        assert_eq!(tipo_vivienda.codigo_postal, TIPO_VIVIENDA_CODIGO_POSTAL);
        assert_eq!(tipo_vivienda.metros_cuadrados, TIPO_VIVIENDA_METROS_CUADRADOS);
        assert_eq!(tipo_vivienda.numero_aseos, TIPO_VIVIENDA_NUMERO_ASEOS);
        assert_eq!(tipo_vivienda.numero_habitaciones, TIPO_VIVIENDA_NUMERO_HABITACIONES);
        assert_eq!(tipo_vivienda.tipo, TIPO_VIVIENDA_TIPO);
    }

    #[test]
    fn should_fail_without_a_calle_or_tipo_vivienda() {

        let err_tipo_vivienda = TipoVivienda::new ( 
            TIPO_VIVIENDA_ID.to_string(),
        "".to_string(),
            TIPO_VIVIENDA_NUMERO,
            TIPO_VIVIENDA_PISO.to_string(),
            TIPO_VIVIENDA_CODIGO_POSTAL.to_string(),
            TIPO_VIVIENDA_METROS_CUADRADOS,
            TIPO_VIVIENDA_NUMERO_ASEOS,
            TIPO_VIVIENDA_NUMERO_HABITACIONES,
            TIPO_VIVIENDA_TIPO);


        assert_eq!(err_tipo_vivienda.is_err(), true);
        assert_eq!(err_tipo_vivienda.unwrap_err(), "Calle no puede estar vacia");
    }
}