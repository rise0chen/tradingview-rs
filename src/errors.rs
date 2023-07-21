use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Wrong username or password")]
    LoginFailed(String),
    #[error("TOTP Error")]
    TOTPError(String),
    #[error("can not parse user data")]
    ParseError(String),
    #[error("Wrong or expired sessionid/signature")]
    SessionExpired,
}
