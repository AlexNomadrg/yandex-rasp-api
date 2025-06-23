use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YaRaspError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("API request 400 Bad Request")]
    ApiBadRequest {
        #[source]
        source: ApiRequestError,
    },

    #[error("API request 404 Not Found")]
    ApiNotFound {
        #[source]
        source: ApiRequestError,
    },

    #[error("API request failed with code {0}")]
    ApiErrorCode(u16),
}

#[derive(Error, Debug, Serialize, Deserialize)]
#[error("{error}")]
pub struct ApiRequestError {
    pub error: ApiErrorBody,
}

#[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[error(
    "Yandex API {http_code}: {text} \
     [uid {error_code}] ↩︎ {request}"
)]
pub struct ApiErrorBody {
    pub request: String,
    pub text: String,
    pub error_code: String,
    pub http_code: i64,
}

impl From<ApiRequestError> for YaRaspError {
    fn from(err: ApiRequestError) -> Self {
        match err.error.http_code {
            400 => YaRaspError::ApiBadRequest { source: err },
            404 => YaRaspError::ApiNotFound { source: err },
            code => YaRaspError::ApiErrorCode(code as u16),
        }
    }
}
