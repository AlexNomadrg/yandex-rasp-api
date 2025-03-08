use chrono::Local;
use yandex_rasp_api::enums::TransportType;
use yandex_rasp_api::YaRaspClient;

#[tokio::main]
async fn main() {
    let client = YaRaspClient::new("Токен");

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
