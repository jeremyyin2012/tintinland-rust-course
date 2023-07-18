use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::resp::Error;

/// 班级
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Class {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub kind: ClassKind,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Copy)]
pub enum ClassKind {
    #[serde(rename = "a")]
    A = 1,
    #[serde(rename = "b")]
    B = 2,
}

impl FromStr for ClassKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown ClassKind".to_string())),
        }
    }
}

impl TryFrom<i32> for ClassKind {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown ClassKind".to_string())),
        }
    }
}


/// 班级-学员
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ClassStudent {
    pub class_id: Uuid,
    pub student_id: Uuid,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}

/// 学员
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Student {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub kind: StudentKind,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Copy)]
pub enum StudentKind {
    #[serde(rename = "a")]
    A = 1,
    #[serde(rename = "b")]
    B = 2,
}

impl FromStr for StudentKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown StudentKind".to_string())),
        }
    }
}

impl TryFrom<i32> for StudentKind {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown StudentKind".to_string())),
        }
    }
}


/// 成绩
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Grades {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub score: i32,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}


/// 教师
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Teacher {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub kind: TeacherKind,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Copy)]
pub enum TeacherKind {
    #[serde(rename = "a")]
    A = 1,
    #[serde(rename = "b")]
    B = 2,
}

impl FromStr for TeacherKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown TeacherKind".to_string())),
        }
    }
}

impl TryFrom<i32> for TeacherKind {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown TeacherKind".to_string())),
        }
    }
}

/// 课程
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Course {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub kind: CourseKind,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Copy)]
pub enum CourseKind {
    #[serde(rename = "a")]
    A = 1,
    #[serde(rename = "b")]
    B = 2,
}

impl FromStr for CourseKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown CourseKind".to_string())),
        }
    }
}

impl TryFrom<i32> for CourseKind {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown CourseKind".to_string())),
        }
    }
}

/// 任课
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Teach {
    pub id: Uuid,
    pub kind: TeachKind,
    pub year: i32,
    pub semester: Semester,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Copy)]
pub enum TeachKind {
    #[serde(rename = "a")]
    A = 1,
    #[serde(rename = "b")]
    B = 2,
}

impl FromStr for TeachKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown TeachKind".to_string())),
        }
    }
}

impl TryFrom<i32> for TeachKind {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown TeachKind".to_string())),
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Copy)]
pub enum Semester {
    #[serde(rename = "a")]
    A = 1,
    #[serde(rename = "b")]
    B = 2,
}

impl FromStr for Semester {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown Semester".to_string())),
        }
    }
}

impl TryFrom<i32> for Semester {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown Semester".to_string())),
        }
    }
}

/// 配课
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Selection {
    pub id: Uuid,
    pub class_id: Uuid,
    pub teach_id: Uuid,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}

/// 上课
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Schedule {
    pub id: Uuid,
    pub selection_id: Uuid,
    pub start_dt: NaiveDateTime,
    pub end_dt: NaiveDateTime,
    pub classroom_id: Uuid,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}


/// 考试
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Exam {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub kind: ExamKind,
    pub selection_id: Uuid,
    pub score: i32,
    pub create_dt: NaiveDateTime,
    pub create_by: Uuid,
    pub update_dt: NaiveDateTime,
    pub update_by: Uuid,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Copy)]
pub enum ExamKind {
    #[serde(rename = "a")]
    A = 1,
    #[serde(rename = "b")]
    B = 2,
}

impl FromStr for ExamKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown ExamKind".to_string())),
        }
    }
}

impl TryFrom<i32> for ExamKind {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            _ => Err(Error::ParamsError("Unknown ExamKind".to_string())),
        }
    }
}
