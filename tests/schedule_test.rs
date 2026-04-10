use chrono::Local;
use tokio;
use yandex_rasp_api::YaRaspClient;

#[tokio::test]
async fn main() {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = YaRaspClient::new(&api_key);

    let station = "s9603402";
    let schedule = client
        .schedule(station)
        .date(Local::now().date_naive())
        .send()
        .await
        .unwrap();
    println!("{:#?}", schedule);
}
