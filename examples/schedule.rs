use chrono::Local;
use yandex_rasp_api::YaRaspClient;

#[tokio::main]
async fn main() {
    let client = YaRaspClient::new("Token");

    let station = "s9603402";
    let schedule = client
        .schedule(station)
        .date(Local::now().date_naive())
        .send()
        .await
        .unwrap();
    println!("{:#?}", schedule);
}
