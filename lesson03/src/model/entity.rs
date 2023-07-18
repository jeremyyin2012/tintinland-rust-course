use std::collections::HashMap;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::resp::Error;


pub type StudentId = Uuid;
pub type ClassId = Uuid;
pub type CourseId = Uuid;
pub type TeacherId = Uuid;
pub type GradesId = Uuid;
pub type TeachId = Uuid;
pub type SelectionId = Uuid;
pub type ScheduleId = Uuid;
pub type ClassroomId = Uuid;
pub type ExamId = Uuid;
pub type UserId = Uuid;


/// 班级
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Class {
    pub id: ClassId,
    pub code: String,
    pub name: String,
    pub kind: ClassKind,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
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
    pub class_id: ClassId,
    pub student_id: StudentId,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
}

/// 学员
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Student {
    pub id: StudentId,
    pub code: String,
    pub name: String,
    pub kind: StudentKind,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
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
    pub id: GradesId,
    pub student_id: StudentId,
    pub course_id: CourseId,
    pub score: i32,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
}


/// 教师
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Teacher {
    pub id: TeacherId,
    pub code: String,
    pub name: String,
    pub kind: TeacherKind,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
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
    pub id: CourseId,
    pub code: String,
    pub name: String,
    pub kind: CourseKind,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
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
    pub id: TeachId,
    pub kind: TeachKind,
    pub year: i32,
    pub semester: Semester,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
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
    pub id: SelectionId,
    pub class_id: ClassId,
    pub teach_id: TeachId,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
}

/// 上课
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Schedule {
    pub id: ScheduleId,
    pub selection_id: SelectionId,
    pub start_dt: NaiveDateTime,
    pub end_dt: NaiveDateTime,
    pub classroom_id: ClassroomId,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
}


/// 考试
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Exam {
    pub id: ExamId,
    pub code: String,
    pub name: String,
    pub kind: ExamKind,
    pub selection_id: SelectionId,
    pub score: i32,
    pub create_dt: NaiveDateTime,
    pub create_by: UserId,
    pub update_dt: NaiveDateTime,
    pub update_by: UserId,
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
