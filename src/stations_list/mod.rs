//! Yandex API Docs: <https://yandex.ru/dev/rasp/doc/ru/reference/stations-list>
mod stations_list_response;
pub use stations_list_response::*;

use crate::enums::Lang;
use crate::errors::YaRaspError;
use crate::{handle_response, YaRaspClient};

pub struct StationsListRequestBuilder {
    ya_rasp_client: YaRaspClient,
    lang: Lang,
}
impl StationsListRequestBuilder {
    pub fn new(ya_rasp_client: YaRaspClient) -> StationsListRequestBuilder {
        Self {
            ya_rasp_client,
            lang: Lang::default(),
        }
    }

    /// Send the request
    pub async fn send(self) -> Result<StationsListResponse, YaRaspError> {
        let response = self
            .ya_rasp_client
            .reqwest_client
            .get("https://api.rasp.yandex.net/v3.0/stations_list/")
            .query(&[
                ("format", "json"),
                ("apikey", &self.ya_rasp_client.api_key),
                ("lang", &self.lang.to_string()),
            ])
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json::<StationsListResponse>().await?)
        } else {
            handle_response::handle_response(response).await
        }
    }

    pub fn lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }
}
