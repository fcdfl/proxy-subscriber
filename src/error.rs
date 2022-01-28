use axum::response::IntoResponse;

#[derive(Debug)]
pub enum AppErrorType {
    Db,
    Template,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub types: AppErrorType,
}

impl AppError {
    pub fn new(
        message: Option<String>,
        cause: Option<Box<dyn std::error::Error>>,
        types: AppErrorType,
    ) -> Self {
        Self {
            message,
            cause,
            types,
        }
    }
    pub fn from_err(cause: Box<dyn std::error::Error>, types: AppErrorType) -> Self {
        Self::new(None, Some(cause), types)
    }
    pub fn from_str(msg: &str, types: AppErrorType) -> Self {
        Self::new(Some(msg.to_string()), None, types)
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Template)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = match self.message {
            Some(msg) => msg,
            None => String::from("有错误发生"),
        };
        msg.into_response()
    }
}
