use yandex_rasp_api::YaRaspClient;

#[tokio::main]
async fn main() {
    let client = YaRaspClient::new("Token");
    let stations_list = client.stations_list().send().await.unwrap();
    println!("{:#?}", stations_list);
}
