use crate::resp::Error;
use crate::resp::Error::NotImplemented;
use crate::services::Services;
use crate::store::Store;

/// 班级
pub struct ClassBusiness {
    svc: Services,
}

impl ClassBusiness {
    pub fn new(store: Store) -> Self {
        Self {
            svc: Services::new(store),
        }
    }
}


impl ClassBusiness {
}