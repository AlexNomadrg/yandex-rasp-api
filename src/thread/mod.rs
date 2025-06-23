mod thread_response;

use crate::enums::{CodeSystem, Lang};
use crate::errors::YaRaspError;
use crate::handle_response::handle_response;
use crate::thread::thread_response::ThreadResponse;
use crate::YaRaspClient;
use chrono::{Local, NaiveDate};

pub struct ThreadRequestBuilder {
    ya_rasp_client: YaRaspClient,
    uid: String,
    from: String,
    to: String,
    lang: Lang,
    date: NaiveDate,
    show_systems: CodeSystem,
}

impl ThreadRequestBuilder {
    pub fn new(ya_rasp_client: YaRaspClient, uid: &str) -> Self {
        Self {
            ya_rasp_client,
            uid: uid.to_string(),
            from: "".to_string(),
            to: "".to_string(),
            lang: Lang::default(),
            date: Local::now().date_naive(),
            show_systems: CodeSystem::Yandex,
        }
    }

    pub async fn send(&self) -> Result<ThreadResponse, YaRaspError> {
        let response = self
            .ya_rasp_client
            .reqwest_client
            .get("https://api.rasp.yandex.net/v3.0/thread")
            .query(&[
                ("format", "json"),
                ("apikey", &self.ya_rasp_client.api_key),
                ("uid", &self.uid),
                ("from", &self.from),
                ("to", &self.to),
                ("lang", &self.lang.to_string()),
                ("date", &self.date.to_string()),
                ("show_systems", &self.show_systems.to_string()),
            ])
            .send()
            .await?;

        handle_response::<ThreadResponse>(response).await
    }

    pub fn from(&mut self, station_id: &str) -> &mut Self {
        self.from = station_id.to_owned();
        self
    }
    pub fn to(&mut self, station_id: &str) -> &mut Self {
        self.to = station_id.to_owned();
        self
    }

    pub fn lang(&mut self, lang: Lang) -> &mut Self {
        self.lang = lang;
        self
    }

    pub fn data(&mut self, date: NaiveDate) -> &mut Self {
        self.date = date;
        self
    }

    pub fn show_systems(&mut self, code_system: CodeSystem) -> &mut Self {
        self
    }
}
