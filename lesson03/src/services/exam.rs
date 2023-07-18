use chrono::{Datelike, Months, NaiveDate, NaiveDateTime, Utc};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use serde_json::Value;

use crate::resp::Error;
use crate::store::api_client::ApiClients;
use crate::store::cache::Caches;
use crate::store::database::Databases;
use crate::store::Store;

/// 考试
pub struct ExamService {
    store: Store,
    db: Databases,
    cache: Caches,
    api: ApiClients,
}


impl ExamService {
    pub fn new(store: Store) -> Self {
        Self {
            store: store.clone(),
            db: store.databases.clone(),
            cache: store.caches.clone(),
            api: store.api_clients.clone(),
        }
    }
}

impl ExamService {
}
