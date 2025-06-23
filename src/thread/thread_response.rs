use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct ThreadResponse {
    pub except_days: String,
    pub arrival_date: Option<String>,
    pub from: Option<String>,
    pub uid: String,
    pub title: String,
    pub departure_date: Option<String>,
    pub number: String,
    pub short_title: String,
    pub days: String,
    pub to: Option<String>,
    pub carrier: Carrier,
    pub transport_type: String,
    pub stops: Vec<Stop>,
    pub vehicle: Option<String>,
    pub start_time: NaiveTime,
    pub start_date: NaiveDate,
    pub transport_subtype: TransportSubtype,
    pub express_type: Option<String>,
    pub interval: Option<Interval>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Interval {
    pub density: String,
    pub end_time: String,
    pub begin_time: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Carrier {
    pub code: i64,
    pub codes: Codes,
    pub title: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Codes {
    pub icao: Option<String>,
    pub sirena: Option<String>,
    pub iata: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Stop {
    pub arrival: Option<String>,
    pub departure: Option<String>,
    pub terminal: Option<String>,
    pub platform: String,
    pub station: Station,
    pub stop_time: Option<f64>,
    pub duration: f64,
}

#[derive(Default, Debug, Deserialize)]
pub struct Station {
    pub codes: Codes,
    pub title: String,
    pub popular_title: Option<String>,
    pub short_title: Option<String>,
    pub station_type: Option<String>,
    pub code: String,
    #[serde(rename = "type")]
    pub station_kind: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct TransportSubtype {
    pub color: Option<String>,
    pub code: Option<String>,
    pub title: Option<String>,
}
