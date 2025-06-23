use tokio;
use yandex_rasp_api::YaRaspClient;

#[tokio::test]
async fn main() {
    let api_key = std::env::var("API_KEY").unwrap();
    let client = YaRaspClient::new(&api_key);

    let thread_from = "6013_0_9600701_g25_4";
    let thread = client.thread(thread_from).send().await.unwrap();
    println!("{:#?}", thread);
}
