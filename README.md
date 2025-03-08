# yandex-rasp-api
### Библиотека для взаимодействия с API [Яндекс.Расписаний](https://rasp.yandex.ru/) на языке Rust

---

> To use this crate you need an API token. [You can get it here](https://yandex.ru/dev/rasp/doc/ru/concepts/access)


# Code Example
```rust
use yandex_rasp_api::enums::TransportTypes;
use yandex_rasp_api::{YaRaspClient, SearchResponse, StationsListResponse};

#[tokio::main]
async fn main() {
    let client: YaRaspClient = YaRaspClient::new("[ВАШ токен API Яндекс.Расписаний]");

    // Получение списка всех станций из API
    let stations_list: StationsListResponse = client
        .stations_list()
        .send()
        .await
        .unwrap();

    // Поиск маршрута между двумя станциями по их коду
    let from = "s9603402";
    let to = "s9602675";
    let search: SearchResponse = client
        .search(&from, &to)
        // Параметры запроса устанавливаются с помощью функций с соответствующим именем
        .transport_types(TransportTypes::Suburban) 
        .send()
        .await
        .unwrap();
}
```
