use crate::services::ping::PingService;
use crate::services::class::ClassService;
use crate::services::course::CourseService;
use crate::services::exam::ExamService;
use crate::services::grades::GradesService;
use crate::services::schedule::ScheduleService;
use crate::services::selection::SelectionService;
use crate::services::student::StudentService;
use crate::services::teach::TeachService;
use crate::services::teacher::TeacherService;
use crate::store::Store;

mod ping;
mod class;
mod course;
mod exam;
mod grades;
mod schedule;
mod selection;
mod student;
mod teach;
mod teacher;


pub struct Services {
    pub ping: PingService,
    // 班级
    pub class: ClassService,
    // 课程
    pub course: CourseService,
    // 考试
    pub exam: ExamService,
    // 成绩
    pub grades: GradesService,
    // 上课
    pub schedule: ScheduleService,
    // 配课
    pub selection: SelectionService,
    // 学员
    pub student: StudentService,
    // 任课
    pub teach: TeachService,
    // 教师
    pub teacher: TeacherService,
}

impl Services {
    pub fn new(store: Store) -> Self {
        Self {
            ping: PingService::new(store.clone()),
            class: ClassService::new(store.clone()),
            course: CourseService::new(store.clone()),
            exam: ExamService::new(store.clone()),
            grades: GradesService::new(store.clone()),
            schedule: ScheduleService::new(store.clone()),
            selection: SelectionService::new(store.clone()),
            student: StudentService::new(store.clone()),
            teach: TeachService::new(store.clone()),
            teacher: TeacherService::new(store.clone()),
        }
    }
}
