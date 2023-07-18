use crate::resp::Error;
use crate::resp::Error::NotImplemented;
use crate::services::Services;
use crate::store::Store;

/// 考试
pub struct ExamBusiness {
    svc: Services,
}

impl ExamBusiness {
    pub fn new(store: Store) -> Self {
        Self {
            svc: Services::new(store),
        }
    }
}


impl ExamBusiness {
}