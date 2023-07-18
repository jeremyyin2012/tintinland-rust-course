use crate::resp::Error;
use crate::resp::Error::NotImplemented;
use crate::services::Services;
use crate::store::Store;

/// 上课
pub struct ScheduleBusiness {
    svc: Services,
}

impl ScheduleBusiness {
    pub fn new(store: Store) -> Self {
        Self {
            svc: Services::new(store),
        }
    }
}


impl ScheduleBusiness {
}