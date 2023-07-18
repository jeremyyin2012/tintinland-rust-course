use crate::resp::Error;
use crate::resp::Error::NotImplemented;
use crate::services::Services;
use crate::store::Store;

/// 教师
pub struct TeacherBusiness {
    svc: Services,
}

impl TeacherBusiness {
    pub fn new(store: Store) -> Self {
        Self {
            svc: Services::new(store),
        }
    }
}


impl TeacherBusiness {
}