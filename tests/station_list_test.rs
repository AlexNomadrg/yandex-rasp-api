use tokio;
use yandex_rasp_api;

#[tokio::test]
async fn test() {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = yandex_rasp_api::YaRaspClient::new(&api_key);
    client.stations_list().send().await.unwrap();
}
