use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use uuid::Uuid;

use crate::business::Business;
use crate::model;
use crate::model::{AddStudent, GetStudentInfo, GetStudentList, Pagination, StudentInfo, StudentList};
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

/// # 查询学员列表
#[openapi(tag = "Student")]
#[get("/api/student/list?<page>&<per_page>")]
pub async fn get_student_list(
    biz: &State<Business>,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Result<Resp<StudentList>, Error> {
    let p = Pagination::from_req(page, per_page);
    let req = GetStudentList {
        pagination: p,
    };
    let res = biz.student.get_student_list(req).await?;
    Ok(Resp::data(res))
}


/// # 查询学员详情
#[openapi(tag = "Student")]
#[get("/api/student/info?<id>")]
pub async fn get_student_info(
    biz: &State<Business>,
    id: Option<Uuid>,
) -> Result<Resp<StudentList>, Error> {
    // todo
    Err(NotImplemented)
}

/// # 添加学员
#[openapi(tag = "Student")]
#[post("/api/student/add", data = "<body>")]
pub async fn add_student(
    biz: &State<Business>,
    body: Json<AddStudent>,
) -> Result<Resp<StudentInfo>, Error> {
    let req = body.into_inner();
    let res = biz.student.add_student(req).await?;
    Ok(Resp::data(res))
}

