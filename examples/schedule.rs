use chrono::Local;
use yandex_rasp_rs::YaRaspClient;

#[tokio::main]
async fn main() {
    let client = YaRaspClient::new("Токен");

    let station = "s9603402";
    let schedule = client
        .schedule(station)
        .date(Local::now().date_naive())
        .send()
        .await
        .unwrap();
    println!("{:#?}", schedule);
}
