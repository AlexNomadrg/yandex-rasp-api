pub mod enums;
pub mod errors;
mod handle_response;
pub mod schedule;
pub mod search;
pub mod stations_list;

use crate::schedule::ScheduleRequestBuilder;
use crate::search::SearchRequestBuilder;
use crate::stations_list::StationsListRequestBuilder;

/// Main client for interacting with the API
#[derive(Clone)]
pub struct YaRaspClient {
    api_key: String,
    reqwest_client: reqwest::Client,
}

impl YaRaspClient {
    /// Create a new client instance using the provided API key
    pub fn new(api_key: &str) -> YaRaspClient {
        Self {
            api_key: String::from(api_key),
            reqwest_client: reqwest::Client::new(),
        }
    }

    /// Returns a request builder for searching the timetable between the provided station codes
    /// Yandex API Docs: <https://yandex.ru/dev/rasp/doc/ru/reference/schedule-point-point>
    pub fn search(&self, from: &str, to: &str) -> SearchRequestBuilder {
        SearchRequestBuilder::new(self.clone(), from.to_string(), to.to_string())
    }

    /// Returns a request builder for searching the timetable for the station whose code was provided
    /// Yandex API Docs: <https://yandex.ru/dev/rasp/doc/ru/reference/schedule-on-station>
    pub fn schedule(&self, station: &str) -> ScheduleRequestBuilder {
        ScheduleRequestBuilder::new(self.clone(), station.to_string())
    }

    /// Returns a request builder for retrieving the list of all stations from the API.
    /// Can be used to find a station code by its name
    /// Yandex API Docs: <https://yandex.ru/dev/rasp/doc/ru/reference/stations-list>
    pub fn stations_list(&self) -> StationsListRequestBuilder {
        StationsListRequestBuilder::new(self.clone())
    }
}
