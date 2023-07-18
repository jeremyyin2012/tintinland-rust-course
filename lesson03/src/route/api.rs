use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use uuid::Uuid;

use crate::business::Business;
use crate::model;
use crate::model::{GetStudentInfo, GetStudentList, Pagination, StudentList};
use crate::resp::Error::{NotImplemented, ParamsError};
use crate::resp::{Error, Resp};


/// # 班级列表
#[openapi(tag = "Class")]
#[get("/api/class/list?<page>&<per_page>")]
pub async fn get_class_list(
    biz: &State<Business>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<Resp<StudentList>, Error> {
    // todo
    Err(NotImplemented)
}


/// # 班级详情
#[openapi(tag = "Class")]
#[get("/api/class/info?<id>")]
pub async fn get_class_info(
    biz: &State<Business>,
    id: Option<Uuid>,
) -> Result<Resp<StudentList>, Error> {
    // todo
    Err(NotImplemented)
}

/// # 学员列表
#[openapi(tag = "Student")]
#[get("/api/student/list?<page>&<per_page>")]
pub async fn get_student_list(
    biz: &State<Business>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<Resp<StudentList>, Error> {
    let p = Pagination::from_req(page, per_page);
    let req = GetStudentList {
        pagination: p,
    };
    let res = biz.student.get_student_list(req).await?;
    Ok(Resp::data(res))
}


/// # 学员详情
#[openapi(tag = "Student")]
#[get("/api/student/info?<id>")]
pub async fn get_student_info(
    biz: &State<Business>,
    id: Option<Uuid>,
) -> Result<Resp<StudentList>, Error> {
    // todo
    Err(NotImplemented)
}

