use std::backtrace::Backtrace;
use std::io;
use std::string::ToString;

use okapi::openapi3::{MediaType, RefOr, Responses};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::{serde_json, Value};
use rocket::tokio::task;
use rocket::{response, Request, Response};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use schemars::_serde_json::json;
use schemars::schema::SchemaObject;
use thiserror::Error;

const PROJECT_NAME: &str = "lesson03";

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Copy, Clone)]
pub struct Placeholder {}

#[derive(Error, Debug, serde::Serialize, schemars::JsonSchema, Copy, Clone)]
pub enum Code {
    #[error("示例")]
    Example = 10000,
    #[error("未找到数据")]
    DataNotFound,
    #[error("缓存中未找到数据")]
    CacheDataNotFound,
    #[error("未知的软件版本")]
    EditionUnknown,
    #[error("未知的平台版本")]
    PlatformUnknown,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("此功能暂未实现")]
    NotImplemented,
    #[error("未认证")]
    Unauthorized,
    #[error("无权限")]
    Forbidden,
    #[error("参数错误: {0}")]
    ParamsError(String),
    #[error("服务报错: {0}")]
    ServerError(String),
    #[error("数据库连接报错: 无法获取数据库连接: {0}")]
    DatabaseConnectionError(String),
    #[error("数据库Ping报错: {0}")]
    DatabasePingError(String),
    #[error("上游服务报错: {0}")]
    UpstreamError(String),
    #[error("服务反馈: {0}")]
    Feedback(Code),
    // 以下是由 thiserror 提供的自动错误转换
    #[error("EnvVarError: {source}, Backtrace: {backtrace}")]
    EnvVarError {
        #[from]
        source: std::env::VarError,
        backtrace: Backtrace,
    },
    #[error("UuidError: {source}, Backtrace: {backtrace}")]
    UuidError {
        #[from]
        source: uuid::Error,
        backtrace: Backtrace,
    },
    #[error("ChronoParseError: {source}, Backtrace: {backtrace}")]
    ChronoParseError {
        #[from]
        source: chrono::ParseError,
        backtrace: Backtrace,
    },
    #[error("RedisError: {source}, Backtrace: {backtrace}")]
    RedisError {
        #[from]
        source: redis::RedisError,
        backtrace: Backtrace,
    },
    #[error("SerdeJsonError: {source}, Backtrace: {backtrace}")]
    SerdeJsonError {
        #[from]
        source: serde_json::Error,
        backtrace: Backtrace,
    },
    #[error("UrlError: {source}, Backtrace: {backtrace}")]
    UrlError {
        #[from]
        source: url::ParseError,
        backtrace: Backtrace,
    },
    #[error("ReqwestError: {source}, Backtrace: {backtrace}")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
        backtrace: Backtrace,
    },
    #[error("SqlxError: {source}, Backtrace: {backtrace}")]
    SqlxError {
        #[from]
        source: sqlx::Error,
        backtrace: Backtrace,
    },
    #[error("SeaQueryError: {source}, Backtrace: {backtrace}")]
    SeaQueryError {
        #[from]
        source: sea_query::error::Error,
        backtrace: Backtrace,
    },
    #[error("IOError: {source}, Backtrace: {backtrace}")]
    IOError {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },
    #[error("TaskJoinError: {source}, Backtrace: {backtrace}")]
    TaskJoinError {
        #[from]
        source: task::JoinError,
        backtrace: Backtrace,
    },
    // 其他任何错误
    #[error("Other: {source}, Backtrace: {backtrace}")]
    Other {
        #[from]
        source: anyhow::Error,
        backtrace: Backtrace,
    },
}

impl Error {
    fn get_http_status(&self) -> Status {
        match self {
            Error::Unauthorized => Status::Unauthorized,
            Error::Forbidden => Status::Forbidden,
            Error::ParamsError(_) => Status::BadRequest,
            Error::ServerError(_) => Status::InternalServerError,
            Error::DatabaseConnectionError(_) => Status::ServiceUnavailable,
            Error::DatabasePingError(_) => Status::ServiceUnavailable,
            Error::UpstreamError(_) => Status::InternalServerError,
            Error::Feedback(_) => Status::Ok,
            _ => Status::InternalServerError,
        }
    }

    fn get_code(&self) -> u16 {
        match self {
            Error::Feedback(code) => (*code).to_owned() as u16,
            _ => self.get_http_status().code,
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let resultcode = self.get_code();
        let errormsg = self.to_string();
        println!(
            "Error: Request: {} Response: {} {}",
            req, resultcode, errormsg
        );
        // 发往sentry的如果有Backtrace则会包含Backtrace，返回给用户的则一定不会有Backtrace
        sentry::capture_error(&self);
        let resp: Resp<Option<String>> = Resp {
            result: false,
            resultcode,
            resultfrom: PROJECT_NAME.parse().unwrap(),
            msg: "".to_string(),
            errormsg: errormsg
                .split(", Backtrace")
                .next()
                .unwrap_or_default()
                .parse()
                .unwrap(),
            data: None,
        };
        let err_response = serde_json::to_string(&resp).unwrap();
        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(err_response.len(), std::io::Cursor::new(err_response))
            .ok()
    }
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct Resp<T> {
    pub result: bool,
    pub resultcode: u16,
    pub resultfrom: String,
    pub msg: String,
    pub errormsg: String,
    pub data: Option<T>,
}

impl<T> Resp<T> {
    pub fn with(data: T, msg: String) -> Self {
        Resp {
            result: true,
            resultcode: 200,
            resultfrom: PROJECT_NAME.parse().unwrap(),
            msg,
            errormsg: "".to_string(),
            data: Some(data),
        }
    }
    pub fn data(data: T) -> Self {
        Resp {
            result: true,
            resultcode: 200,
            resultfrom: PROJECT_NAME.parse().unwrap(),
            msg: "".to_string(),
            errormsg: "".to_string(),
            data: Some(data),
        }
    }
    pub fn msg(msg: String) -> Self {
        Resp {
            result: true,
            resultcode: 200,
            resultfrom: PROJECT_NAME.parse().unwrap(),
            msg,
            errormsg: "".to_string(),
            data: None,
        }
    }
    pub fn ok() -> Self {
        Resp {
            result: true,
            resultcode: 200,
            resultfrom: PROJECT_NAME.parse().unwrap(),
            msg: "OK".to_string(),
            errormsg: "".to_string(),
            data: None,
        }
    }
}

impl<'r, T: serde::Serialize> Responder<'r, 'static> for Resp<T> {
    fn respond_to(self, _req: &'r Request) -> response::Result<'static> {
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::JSON)
            .status(Status::new(self.resultcode))
            .ok()
    }
}

pub fn response_ok(_gen: &mut OpenApiGenerator, schema: SchemaObject) -> okapi::openapi3::Response {
    okapi::openapi3::Response {
        description: "OK".to_string(),
        content: okapi::map! {
            "application/json".to_owned() => MediaType{
                schema: Some(schema),
                ..Default::default()
            }
        },
        ..Default::default()
    }
}

pub fn response_err(
    _gen: &mut OpenApiGenerator,
    schema: SchemaObject,
    desc: String,
    example: Option<Value>,
) -> okapi::openapi3::Response {
    okapi::openapi3::Response {
        description: desc.to_owned(),
        content: okapi::map! {
            "application/json".to_owned() => MediaType{
                schema: Some(schema),
                example: example,
                ..Default::default()
            }
        },
        ..Default::default()
    }
}

impl<T: schemars::JsonSchema> OpenApiResponderInner for Resp<T> {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let schema = gen.json_schema::<Resp<T>>();
        Ok(Responses {
            responses: okapi::map! {
                "200 OK".to_owned() => RefOr::Object(response_ok(gen, schema))
            },
            ..Default::default()
        })
    }
}

impl OpenApiResponderInner for Error {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let schema = gen.json_schema::<Resp<Option<String>>>();
        Ok(Responses {
            responses: okapi::map! {
                Error::Feedback(Code::Example).get_http_status().to_string() + ": Feedback" => RefOr::Object(
                response_err(gen, schema.clone(), Error::Feedback(Code::Example).to_string(),
                    Some(json!({
                            "resultfrom": PROJECT_NAME,
                            "resultcode": Code::Example as u16,
                            "result": false,
                            "msg": "",
                            "errormsg": Error::Feedback(Code::Example).to_string(),
                            "data": null
                        })))),

                Error::Unauthorized.get_http_status().to_string() => RefOr::Object(
                response_err(gen, schema.clone(), Error::Unauthorized.to_string(),
                    Some(json!({
                            "resultfrom": PROJECT_NAME,
                            "resultcode": 401,
                            "result": false,
                            "msg": "",
                            "errormsg": Error::Unauthorized.to_string(),
                            "data": null
                        })))),

                Error::Forbidden.get_http_status().to_string() => RefOr::Object(
                response_err(gen, schema.clone(), Error::Forbidden.to_string(),
                    Some(json!({
                            "resultfrom": PROJECT_NAME,
                            "resultcode": 403,
                            "result": false,
                            "msg": "",
                            "errormsg": Error::Forbidden.to_string(),
                            "data": null
                        })))),

                Error::ParamsError("".to_string()).get_http_status().to_string() + ": ParamsError" => RefOr::Object(
                response_err(gen, schema.clone(), Error::ParamsError("".to_string()).to_string(),
                    Some(json!({
                            "resultfrom": PROJECT_NAME,
                            "resultcode": 400,
                            "result": false,
                            "msg": "",
                            "errormsg": Error::ParamsError("".to_string()).to_string(),
                            "data": null
                        })))),

                Error::ServerError("".to_string()).get_http_status().to_string() + ": ServerError" => RefOr::Object(
                response_err(gen, schema.clone(), Error::ServerError("".to_string()).to_string(),
                    Some(json!({
                            "resultfrom": PROJECT_NAME,
                            "resultcode": 500,
                            "result": false,
                            "msg": "",
                            "errormsg": Error::ServerError("".to_string()).to_string(),
                            "data": null
                        })))),

                Error::UpstreamError("".to_string()).get_http_status().to_string() + ": UpstreamError" => RefOr::Object(
                response_err(gen, schema.clone(), Error::UpstreamError("".to_string()).to_string(),
                    Some(json!({
                            "resultfrom": PROJECT_NAME,
                            "resultcode": 500,
                            "result": false,
                            "msg": "",
                            "errormsg": Error::UpstreamError("".to_string()).to_string(),
                            "data": null
                        })))),

                Error::NotImplemented.get_http_status().to_string() => RefOr::Object(
                response_err(gen, schema.clone(), Error::NotImplemented.to_string(),
                    Some(json!({
                            "resultfrom": PROJECT_NAME,
                            "resultcode": 500,
                            "result": false,
                            "msg": "",
                            "errormsg": Error::NotImplemented.to_string(),
                            "data": null
                        })))),

            },
            ..Default::default()
        })
    }
}
