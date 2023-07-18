use crate::resp::Error;
use serde::{Serialize, Serializer};
use std::str::FromStr;
use uuid::Uuid;
use crate::model::{ClassKind, CourseKind, ExamKind, Semester, StudentKind, TeacherKind, TeachKind};

impl Serialize for ClassKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}



impl Serialize for StudentKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}


impl Serialize for TeacherKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}

impl Serialize for CourseKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}


impl Serialize for TeachKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}


impl Serialize for Semester {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}



impl Serialize for ExamKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}