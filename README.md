# yandex-rasp-api
### Library for interacting with the [Yandex Timetable](https://rasp.yandex.ru/) API in Rust

---

> To use this crate you need an API token. [You can get it here](https://yandex.ru/dev/rasp/doc/ru/concepts/access)


# Code Example
```rust
use yandex_rasp_api::enums::TransportTypes;
use yandex_rasp_api::{YaRaspClient, SearchResponse, StationsListResponse};

#[tokio::main]
async fn main() {
    let client: YaRaspClient = YaRaspClient::new("[YOUR Yandex Timetable API token]");

    // Retrieve the list of all stations from the API
    let stations_list: StationsListResponse = client
        .stations_list()
        .send()
        .await
        .unwrap();

    // Search for a route between two stations by their code
    let from = "s9603402";
    let to = "s9602675";
    let search: SearchResponse = client
        .search(&from, &to)
        // Request parameters are set using functions with the corresponding names
        .transport_types(TransportTypes::Suburban) 
        .send()
        .await
        .unwrap();
}
```

Forked from: [KryptonFox/yandex-rasp-rs](https://github.com/KryptonFox/yandex-rasp-rs)
