use chrono::Local;
use tokio;
use yandex_rasp_api::{enums::TransportType, YaRaspClient};

#[tokio::test]
async fn main() {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = YaRaspClient::new(&api_key);

    let from = "s9603402";
    let to = "s9602675";
    let search = client
        .search(from, to)
        .transport_types(TransportType::Suburban)
        .date(Local::now().date_naive())
        .send()
        .await
        .unwrap();
    println!("{:#?}", search);
}
