use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::model::{Class, Course, Exam, Grades, Schedule, Selection, Student, StudentKind, Teach, Teacher};
use crate::resp::Error;

/// 分页
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
    pub total_count: i32,
    pub total_pages: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
            total_count: 0,
            total_pages: 0,
        }
    }
}

impl Pagination {
    pub fn calculate_total_pages(&mut self, total_count: i32) -> &Self {
        let total_pages= total_count.clone() as f64 / self.per_page.clone() as f64;
        self.total_count = total_count;
        self.total_pages = total_pages.ceil() as i32;
        self
    }
    
    pub fn from_req(page: Option<i32>, per_page: Option<i32>) -> Self {
        Self {
            page: page.unwrap_or(1),
            per_page: per_page.unwrap_or(10),
            total_count: 0,
            total_pages: 0,
        }
    }
}

/// 学员列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct StudentList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Student>,
}

/// 学员详情
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct StudentInfo {

}

/// 教师列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct TeacherList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Teacher>,
}


/// 课程列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CourseList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Course>,
}


/// 班级列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ClassList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Class>,
}


/// 成绩列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GradesList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Grades>,
}


/// 任课列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct TeachList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Teach>,
}

/// 配课列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SelectionList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Selection>,
}

/// 上课列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ScheduleList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Schedule>,
}

/// 考试列表
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ExamList {
    #[serde(flatten)]
    pub pagination: Pagination,
    pub list: Vec<Exam>,
}
