use crate::errors::{ApiRequestError, YaRaspError};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

/// Convert an HTTP response into either the success type `T`
/// or one of the structured `YaRaspError` variants.
pub async fn handle_response<T>(response: Response) -> Result<T, YaRaspError>
where
    T: DeserializeOwned,
{
    match response.status() {
        StatusCode::OK => Ok(response.json::<T>().await?),

        StatusCode::BAD_REQUEST => {
            let err_body = response.json::<ApiRequestError>().await?;
            Err(YaRaspError::ApiBadRequest { source: err_body })
        }

        StatusCode::NOT_FOUND => {
            let err_body = response.json::<ApiRequestError>().await?;
            Err(YaRaspError::ApiNotFound { source: err_body })
        }

        other => Err(YaRaspError::ApiErrorCode(other.as_u16())),
    }
}
