use crate::error::AppError;

pub async fn root_handler() -> Result<&'static str, AppError> {
    Ok("hello world from the rust http server")
}