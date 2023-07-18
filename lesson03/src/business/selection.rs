use crate::resp::Error;
use crate::resp::Error::NotImplemented;
use crate::services::Services;
use crate::store::Store;

/// 配课
pub struct SelectionBusiness {
    svc: Services,
}

impl SelectionBusiness {
    pub fn new(store: Store) -> Self {
        Self {
            svc: Services::new(store),
        }
    }
}


impl SelectionBusiness {
}