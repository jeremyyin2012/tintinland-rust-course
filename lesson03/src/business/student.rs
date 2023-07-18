use crate::model::{AddStudent, GetStudentInfo, GetStudentList, Pagination, StudentInfo, StudentList};
use crate::resp::Error;
use crate::resp::Error::NotImplemented;
use crate::services::Services;
use crate::store::Store;

/// 学员
pub struct StudentBusiness {
    svc: Services,
}

impl StudentBusiness {
    pub fn new(store: Store) -> Self {
        Self {
            svc: Services::new(store),
        }
    }
}


impl StudentBusiness {
    pub async fn get_student_list(&self, req: GetStudentList) -> Result<StudentList, Error> {
        let (rows, total_count) = self.svc.student.get_students_by_filters(req.clone()).await?;
        let mut p = req.pagination.clone();
        p.calculate_total_pages(total_count);
        let res = StudentList {
            pagination: p,
            list: rows,
        };
        Ok(res)
    }

    pub async fn get_student_info(&self, req: GetStudentInfo) -> Result<StudentInfo, Error> {
        Err(NotImplemented)
    }

    pub async fn add_student(&self, req: AddStudent) -> Result<StudentInfo, Error> {
        let student = self.svc.student.add_student(req).await?;
        let s = StudentInfo {
            id: student.id,
            code: student.code,
            name: student.name,
            kind: student.kind,
            create_dt: student.create_dt,
            create_by: student.create_by,
            update_dt: student.update_dt,
            update_by: student.update_by,
        };
        Ok(s)
    }
}