use axum::response::IntoResponse;

#[derive(Debug)]
pub enum AppErrorType {
    Db,
    Template,
    NotFound,
    InvalidParamter,
    Yaml,
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
    pub fn not_found_msg_opt(msg: Option<&str>) -> Self {
        let msg = match msg {
            Some(msg) => msg,
            None => "没有满足条件的数据",
        };
        Self::from_str(msg, AppErrorType::NotFound)
    }
    pub fn not_found_msg(msg: &str) -> Self {
        Self::not_found_msg_opt(Some(msg))
    }
    pub fn not_found() -> Self {
        Self::not_found_msg_opt(None)
    }
    pub fn invalid_param_msg(msg: &str) -> Self {
        Self::from_str(msg, AppErrorType::InvalidParamter)
    }
    pub fn invalid_param() -> Self {
        Self::invalid_param_msg("参数错误")
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Template)
    }
}
impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}
impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}
impl From<serde_yaml::Error> for AppError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Yaml)
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
