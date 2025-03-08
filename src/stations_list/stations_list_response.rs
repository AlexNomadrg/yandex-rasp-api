use serde::{Deserialize, Serialize};

use crate::enums::TransportType;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StationsListResponse {
    pub countries: Vec<Country>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub regions: Vec<Region>,
    pub codes: Codes,
    pub title: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub settlements: Vec<Settlement>,
    pub codes: Codes,
    pub title: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub title: String,
    pub codes: Codes,
    pub stations: Vec<Station>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(untagged)]
pub enum LongOrLatItude {
    #[default]
    None,
    Float(f64),
    EmptyString(String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub title: String,
    pub direction: String,
    pub codes: Codes,
    pub station_type: String,
    pub transport_type: TransportType,
    pub longitude: LongOrLatItude,
    pub latitude: LongOrLatItude,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Codes {
    pub yandex_code: Option<String>,
    pub esr_code: Option<String>,
}
