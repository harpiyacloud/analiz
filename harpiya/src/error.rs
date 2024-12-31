#[derive(Debug)]
pub struct ApiError { }

impl From<aes_gcm_siv::Error> for ApiError {
    fn from(_err: aes_gcm_siv::Error) -> Self {
        Self { }
    }
}

impl From<base64::DecodeError> for ApiError {
    fn from(_err: base64::DecodeError) -> Self {
        Self {  }
    }
}

impl From<argon2::password_hash::Error> for ApiError {
    fn from(_err: argon2::password_hash::Error) -> Self {
        Self {  }
    }
}

impl From<ApiError> for tonic::Status {
    fn from(_err: ApiError) -> Self {
        Self::internal("message")
    }
}