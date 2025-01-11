use yandex_rasp_rs::YaRaspClient;

#[tokio::main]
async fn main() {
    let client = YaRaspClient::new("Токен");
    let stations_list = client.stations_list().send().await.unwrap();
    println!("{:#?}", stations_list);
}
