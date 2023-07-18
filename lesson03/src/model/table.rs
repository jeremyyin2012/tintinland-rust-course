// sea-sql

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use rocket::http::ext::IntoOwned;
use sea_query::Iden;
use sqlx::{Database, Decode};
use sqlx::database::HasValueRef;
use uuid::Uuid;

use crate::model::{Class, ClassKind, Course, CourseKind, Exam, ExamKind, Schedule, Selection, Semester, Student, StudentKind, Teach, TeachKind, Teacher, TeacherKind, ClassStudent, Grades};
use crate::resp::Error;

#[derive(sqlx::Type)]
#[sqlx(transparent)]
#[derive(
Clone,
Debug,
Default,
Eq,
Hash,
Ord,
PartialEq,
PartialOrd,
serde::Serialize,
serde::Deserialize,
schemars::JsonSchema,
)]
#[repr(transparent)]
pub struct MyUuid(String);

impl MyUuid {
    pub fn to_uuid(&self) -> Result<Uuid, Error> {
        Ok(Uuid::parse_str(self.0.as_str())?)
    }

    pub fn from_uuid(s: Uuid) -> Self {
        MyUuid(s.simple().to_string())
    }
}


/// 班级
#[derive(Iden)]
#[iden = "class"]
pub enum TClass {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "code"]
    Code,
    #[iden = "name"]
    Name,
    #[iden = "kind"]
    Kind,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TClass {
    pub fn cols_all() -> [Self; 8] {
        [
            Self::Id,
            Self::Code,
            Self::Name,
            Self::Kind,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTClass {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "code")]
    pub code: String,
    #[sqlx(rename = "name")]
    pub name: String,
    #[sqlx(rename = "kind")]
    pub kind: i32,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTClass {
    pub fn to_entity(&self) -> Result<Class, Error> {
        let entity = Class {
            id: self.id,
            code: self.code.clone(),
            name: self.name.clone(),
            kind: ClassKind::try_from(self.kind.clone())?,
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}


/// 班级-学员
#[derive(Iden)]
#[iden = "class_student"]
pub enum TClassStudent {
    Table,
    #[iden = "class_id"]
    ClassId,
    #[iden = "student_id"]
    StudentId,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TClassStudent {
    pub fn cols_all() -> [Self; 6] {
        [
            Self::ClassId,
            Self::StudentId,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTClassStudent {
    #[sqlx(rename = "class_id")]
    pub class_id: Uuid,
    #[sqlx(rename = "student_id")]
    pub student_id: Uuid,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTClassStudent {
    pub fn to_entity(&self) -> Result<ClassStudent, Error> {
        let entity = ClassStudent {
            class_id: self.class_id,
            student_id: self.student_id,
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}

/// 学员
#[derive(Iden)]
#[iden = "student"]
pub enum TStudent {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "code"]
    Code,
    #[iden = "name"]
    Name,
    #[iden = "kind"]
    Kind,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TStudent {
    pub fn cols_all() -> [Self; 8] {
        [
            Self::Id,
            Self::Code,
            Self::Name,
            Self::Kind,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTStudent {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "code")]
    pub code: String,
    #[sqlx(rename = "name")]
    pub name: String,
    #[sqlx(rename = "kind")]
    pub kind: i32,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTStudent {
    pub fn to_entity(&self) -> Result<Student, Error> {
        let entity = Student {
            id: self.id,
            code: self.code.clone(),
            name: self.name.clone(),
            kind: StudentKind::try_from(self.kind.clone())?,
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}


/// 教师
#[derive(Iden)]
#[iden = "teacher"]
pub enum TTeacher {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "code"]
    Code,
    #[iden = "name"]
    Name,
    #[iden = "kind"]
    Kind,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TTeacher {
    pub fn cols_all() -> [Self; 8] {
        [
            Self::Id,
            Self::Code,
            Self::Name,
            Self::Kind,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTTeacher {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "code")]
    pub code: String,
    #[sqlx(rename = "name")]
    pub name: String,
    #[sqlx(rename = "kind")]
    pub kind: i32,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTTeacher {
    pub fn to_entity(&self) -> Result<Teacher, Error> {
        let entity = Teacher {
            id: self.id,
            code: self.code.clone(),
            name: self.name.clone(),
            kind: TeacherKind::try_from(self.kind.clone())?,
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}


/// 课程
#[derive(Iden)]
#[iden = "course"]
pub enum TCourse {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "code"]
    Code,
    #[iden = "name"]
    Name,
    #[iden = "kind"]
    Kind,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TCourse {
    pub fn cols_all() -> [Self; 8] {
        [
            Self::Id,
            Self::Code,
            Self::Name,
            Self::Kind,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTCourse {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "code")]
    pub code: String,
    #[sqlx(rename = "name")]
    pub name: String,
    #[sqlx(rename = "kind")]
    pub kind: i32,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTCourse {
    pub fn to_entity(&self) -> Result<Course, Error> {
        let entity = Course {
            id: self.id,
            code: self.code.clone(),
            name: self.name.clone(),
            kind: CourseKind::try_from(self.kind.clone())?,
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}


/// 考试
#[derive(Iden)]
#[iden = "exam"]
pub enum TExam {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "code"]
    Code,
    #[iden = "name"]
    Name,
    #[iden = "kind"]
    Kind,
    #[iden = "selection_id"]
    Selection,
    #[iden = "score"]
    Score,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TExam {
    pub fn cols_all() -> [Self; 10] {
        [
            Self::Id,
            Self::Code,
            Self::Name,
            Self::Kind,
            Self::Selection,
            Self::Score,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTExam {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "code")]
    pub code: String,
    #[sqlx(rename = "name")]
    pub name: String,
    #[sqlx(rename = "kind")]
    pub kind: i32,
    #[sqlx(rename = "selection_id")]
    pub selection_id: Uuid,
    #[sqlx(rename = "score")]
    pub score: i32,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTExam {
    pub fn to_entity(&self) -> Result<Exam, Error> {
        let entity = Exam {
            id: self.id,
            code: self.code.clone(),
            name: self.name.clone(),
            kind: ExamKind::try_from(self.kind.clone())?,
            selection_id: self.selection_id,
            score: self.score.clone(),
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}


/// 配课
#[derive(Iden)]
#[iden = "selection"]
pub enum TSelection {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "class_id"]
    ClassId,
    #[iden = "teach_id"]
    TeachId,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TSelection {
    pub fn cols_all() -> [Self; 7] {
        [
            Self::Id,
            Self::ClassId,
            Self::TeachId,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTSelection {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "class_id")]
    pub class_id: Uuid,
    #[sqlx(rename = "teach_id")]
    pub teach_id: Uuid,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTSelection {
    pub fn to_entity(&self) -> Result<Selection, Error> {
        let entity = Selection {
            id: self.id,
            class_id: self.class_id,
            teach_id: self.teach_id,
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}


/// 上课
#[derive(Iden)]
#[iden = "schedule"]
pub enum TSchedule {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "selection_id"]
    SelectionId,
    #[iden = "start_dt"]
    StartDt,
    #[iden = "end_dt"]
    EndDt,
    #[iden = "classroom_id"]
    ClassroomId,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TSchedule {
    pub fn cols_all() -> [Self; 9] {
        [
            Self::Id,
            Self::SelectionId,
            Self::StartDt,
            Self::EndDt,
            Self::ClassroomId,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTSchedule {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "selection_id")]
    pub selection_id: Uuid,
    #[sqlx(rename = "start_dt")]
    pub start_dt: NaiveDateTime,
    #[sqlx(rename = "end_dt")]
    pub end_dt: NaiveDateTime,
    #[sqlx(rename = "classroom_id")]
    pub classroom_id: Uuid,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTSchedule {
    pub fn to_entity(&self) -> Result<Schedule, Error> {
        let entity = Schedule {
            id: self.id,
            selection_id: self.selection_id,
            start_dt: self.start_dt,
            end_dt: self.end_dt,
            classroom_id: self.classroom_id,
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}


/// 任课
#[derive(Iden)]
#[iden = "teach"]
pub enum TTeach {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "kind"]
    Kind,
    #[iden = "year"]
    Year,
    #[iden = "semester"]
    Semester,
    #[iden = "course_id"]
    CourseId,
    #[iden = "teacher_id"]
    TeacherId,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TTeach {
    pub fn cols_all() -> [Self; 10] {
        [
            Self::Id,
            Self::Kind,
            Self::Year,
            Self::Semester,
            Self::CourseId,
            Self::TeacherId,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTTeach {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "kind")]
    pub kind: i32,
    #[sqlx(rename = "year")]
    pub year: i32,
    #[sqlx(rename = "semester")]
    pub semester: i32,
    #[sqlx(rename = "course_id")]
    pub course_id: Uuid,
    #[sqlx(rename = "teacher_id")]
    pub teacher_id: Uuid,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTTeach {
    pub fn to_entity(&self) -> Result<Teach, Error> {
        let entity = Teach {
            id: self.id,
            kind: TeachKind::try_from(self.kind.clone())?,
            year: self.year.clone(),
            semester: Semester::try_from(self.semester.clone())?,
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}


/// 成绩
#[derive(Iden)]
#[iden = "grades"]
pub enum TGrades {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "student_id"]
    StudentId,
    #[iden = "course_id"]
    CourseId,
    #[iden = "score"]
    Score,
    #[iden = "create_dt"]
    CreateDt,
    #[iden = "create_by"]
    CreateBy,
    #[iden = "update_dt"]
    UpdateDt,
    #[iden = "update_by"]
    UpdateBy,
}

impl TGrades {
    pub fn cols_all() -> [Self; 8] {
        [
            Self::Id,
            Self::StudentId,
            Self::CourseId,
            Self::Score,
            Self::CreateDt,
            Self::CreateBy,
            Self::UpdateDt,
            Self::UpdateBy,
        ]
    }
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct RowTGrades {
    #[sqlx(rename = "id")]
    pub id: Uuid,
    #[sqlx(rename = "student_id")]
    pub student_id: Uuid,
    #[sqlx(rename = "course_id")]
    pub course_id: Uuid,
    #[sqlx(rename = "score")]
    pub score: i32,
    #[sqlx(rename = "create_dt")]
    pub create_dt: NaiveDateTime,
    #[sqlx(rename = "create_by")]
    pub create_by: Uuid,
    #[sqlx(rename = "update_dt")]
    pub update_dt: NaiveDateTime,
    #[sqlx(rename = "update_by")]
    pub update_by: Uuid,
}

impl RowTGrades {
    pub fn to_entity(&self) -> Result<Grades, Error> {
        let entity = Grades {
            id: self.id,
            student_id: self.student_id,
            course_id: self.course_id,
            score: self.score.clone(),
            create_dt: self.create_dt,
            create_by: self.create_by,
            update_dt: self.update_dt,
            update_by: self.update_by,
        };
        Ok(entity)
    }
}
