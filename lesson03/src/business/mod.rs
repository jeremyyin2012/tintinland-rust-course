use crate::business::ping::PingBusiness;
use crate::business::student::StudentBusiness;
use crate::business::class::ClassBusiness;
use crate::business::teacher::TeacherBusiness;
use crate::business::course::CourseBusiness;
use crate::business::teach::TeachBusiness;
use crate::business::selection::SelectionBusiness;
use crate::business::schedule::ScheduleBusiness;
use crate::business::exam::ExamBusiness;
use crate::business::grades::GradesBusiness;
use crate::store::Store;

mod ping;
mod student;
mod class;
mod teacher;
mod course;
mod teach;
mod selection;
mod schedule;
mod exam;
mod grades;

pub struct Business {
    pub ping: PingBusiness,
    // 学员
    pub student: StudentBusiness,
    // 班级
    pub class: ClassBusiness,
    // 教师
    pub teacher: TeacherBusiness,
    // 课程
    pub course: CourseBusiness,
    // 任课
    pub teach: TeachBusiness,
    // 配课
    pub selection: SelectionBusiness,
    // 上课
    pub schedule: ScheduleBusiness,
    // 考试
    pub exam: ExamBusiness,
    // 成绩
    pub grades: GradesBusiness,
}

impl Business {
    pub fn new(store: Store) -> Self {
        Self {
            ping: PingBusiness::new(store.clone()),
            student: StudentBusiness::new(store.clone()),
            class: ClassBusiness::new(store.clone()),
            teacher: TeacherBusiness::new(store.clone()),
            course: CourseBusiness::new(store.clone()),
            teach: TeachBusiness::new(store.clone()),
            selection: SelectionBusiness::new(store.clone()),
            schedule: ScheduleBusiness::new(store.clone()),
            exam: ExamBusiness::new(store.clone()),
            grades: GradesBusiness::new(store.clone()),
        }
    }
}
