//! Yandex API Docs: <https://yandex.ru/dev/rasp/doc/ru/reference/schedule-on-station>
mod schedule_response;
pub use schedule_response::*;

use crate::enums::{CodeSystem, Lang, ScheduleEvent, TransportType};
use crate::errors::YaRaspError;
use crate::handle_response::handle_response;
use crate::YaRaspClient;
use chrono::{Local, NaiveDate};
use chrono_tz::Tz;
use chrono_tz::Tz::UTC;

pub struct ScheduleRequestBuilder {
    ya_rasp_client: YaRaspClient,
    station: String,
    lang: Lang,
    date: NaiveDate,
    transport_types: TransportType,
    event: ScheduleEvent,
    system: CodeSystem,
    show_systems: CodeSystem,
    direction: String,
    result_timezone: Tz,
}

impl ScheduleRequestBuilder {
    pub fn new(ya_rasp_client: YaRaspClient, station: String) -> Self {
        Self {
            ya_rasp_client,
            station,
            lang: Lang::default(),
            date: Local::now().naive_local().date(),
            transport_types: TransportType::default(),
            system: CodeSystem::Yandex,
            event: ScheduleEvent::default(),
            show_systems: CodeSystem::Yandex,
            direction: String::from("all"),
            result_timezone: UTC,
        }
    }

    /// Send the request
    pub async fn send(self) -> Result<ScheduleResponse, YaRaspError> {
        let response = self
            .ya_rasp_client
            .reqwest_client
            .get("https://api.rasp.yandex.net/v3.0/schedule/")
            .query(&[
                ("format", "json"),
                ("apikey", &self.ya_rasp_client.api_key),
                ("station", &self.station),
                ("lang", &self.lang.to_string()),
                ("date", &self.date.to_string()),
                ("transport_types", &self.transport_types.to_string()),
                ("system", &self.system.to_string()),
                ("event", &self.event.to_string()),
                ("show_systems", &self.show_systems.to_string()),
                ("direction", &self.direction),
                ("result_timezone", &self.result_timezone.to_string()),
            ])
            .send()
            .await?;
        handle_response::<ScheduleResponse>(response).await
    }

    pub fn lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }

    pub fn date(mut self, date: NaiveDate) -> Self {
        self.date = date;
        self
    }

    pub fn transport_types(mut self, transport_type: TransportType) -> Self {
        self.transport_types = transport_type;
        self
    }

    pub fn system(mut self, system: CodeSystem) -> Self {
        self.system = system;
        self
    }

    pub fn direction(mut self, direction: String) -> Self {
        self.direction = direction;
        self
    }

    pub fn show_systems(mut self, system: CodeSystem) -> Self {
        self.show_systems = system;
        self
    }

    pub fn result_timezone(mut self, result_timezone: Tz) -> Self {
        self.result_timezone = result_timezone;
        self
    }
}
