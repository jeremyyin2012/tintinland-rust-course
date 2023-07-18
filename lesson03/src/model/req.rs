use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

use okapi::openapi3::{Object, Parameter, ParameterValue};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use schemars::JsonSchema;
use uuid::uuid;
use uuid::Uuid;
use crate::model::{Pagination, StudentKind};

use crate::resp::Error;
use crate::resp::Error::ParamsError;

/// 学员列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetStudentList {
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// 学员详情
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetStudentInfo {
    pub id: Uuid,
}


/// 学员详情
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct AddStudent {
    pub code: String,
    pub name: String,
    pub kind: StudentKind,
}
