
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Tipo {
    Apartamento,
    Casa,
    Chalet
}

#[derive(Debug, Deserialize,Serialize,Clone)]
pub struct TipoVivienda {
    pub id: String,
    pub calle: String,
    pub numero: i32,
    pub piso: String,
    pub codigo_postal: String,
    pub metros_cuadrados: i32,
    pub numero_aseos: i32,
    pub numero_habitaciones: i32,
    pub tipo: Tipo
}

impl TipoVivienda {
    pub fn new(id: String, calle: String, numero:i32, piso:String, codigo_postal:String, metros_cuadrados:i32, numero_aseos:i32,numero_habitaciones:i32,tipo:Tipo) -> Self {

        Self {
            id, calle, numero, piso, codigo_postal, metros_cuadrados, numero_aseos,numero_habitaciones,tipo
        }
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
            TIPO_VIVIENDA_TIPO);

        assert_eq!(tipo_vivienda.id, TIPO_VIVIENDA_ID);
        assert_eq!(tipo_vivienda.calle, TIPO_VIVIENDA_CALLE);
        assert_eq!(tipo_vivienda.numero, TIPO_VIVIENDA_NUMERO);
        assert_eq!(tipo_vivienda.piso, TIPO_VIVIENDA_PISO);
        assert_eq!(tipo_vivienda.codigo_postal, TIPO_VIVIENDA_CODIGO_POSTAL);
        assert_eq!(tipo_vivienda.metros_cuadrados, TIPO_VIVIENDA_METROS_CUADRADOS);
        assert_eq!(tipo_vivienda.numero_aseos, TIPO_VIVIENDA_NUMERO_ASEOS);
        assert_eq!(tipo_vivienda.numero_habitaciones, TIPO_VIVIENDA_NUMERO_HABITACIONES);
        assert_eq!(tipo_vivienda.tipo, TIPO_VIVIENDA_TIPO);
    }
}