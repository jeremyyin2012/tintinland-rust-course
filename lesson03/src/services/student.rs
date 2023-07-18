use chrono::{Datelike, Months, NaiveDate, NaiveDateTime, Utc};
use sea_query::{Expr, Func, PostgresQueryBuilder, Query};
use serde_json::Value;
use sqlx::Row;
use uuid::Uuid;

use crate::model::{AddStudent, GetStudentList, RowTStudent, Student, StudentId, TStudent};
use crate::resp::Error;
use crate::resp::Error::NotImplemented;
use crate::store::api_client::ApiClients;
use crate::store::cache::Caches;
use crate::store::database::Databases;
use crate::store::Store;

/// 学生
pub struct StudentService {
    store: Store,
    db: Databases,
    cache: Caches,
    api: ApiClients,
}


impl StudentService {
    pub fn new(store: Store) -> Self {
        Self {
            store: store.clone(),
            db: store.databases.clone(),
            cache: store.caches.clone(),
            api: store.api_clients.clone(),
        }
    }
}

impl StudentService {
    pub async fn get_student_by_id(&self, student_id: StudentId) -> Result<Student, Error> {
        let sql = Query::select()
            .columns(TStudent::cols_all())
            .from(TStudent::Table)
            .and_where(
                Expr::col(TStudent::Id).eq(student_id),
            )
            .to_string(PostgresQueryBuilder);
        let mut conn = self.db.default.acquire().await?;
        let row: RowTStudent = sqlx::query_as(&sql).fetch_one(&mut conn).await?;
        let entity = row.to_entity()?;
        Ok(entity)
    }

    pub async fn get_student_by_code(&self, code: String) -> Result<Student, Error> {
        let sql = Query::select()
            .columns(TStudent::cols_all())
            .from(TStudent::Table)
            .and_where(
                Expr::col(TStudent::Code).eq(code),
            )
            .to_string(PostgresQueryBuilder);
        let mut conn = self.db.default.acquire().await?;
        let row: RowTStudent = sqlx::query_as(&sql).fetch_one(&mut conn).await?;
        let entity = row.to_entity()?;
        Ok(entity)
    }

    pub async fn get_students_by_ids(&self, student_ids: Vec<StudentId>) -> Result<Vec<Student>, Error> {
        let sql = Query::select()
            .columns(TStudent::cols_all())
            .from(TStudent::Table)
            .and_where(
                Expr::col(TStudent::Id).is_in(student_ids),
            )
            .to_string(PostgresQueryBuilder);
        let mut conn = self.db.default.acquire().await?;
        let rows: Vec<RowTStudent> = sqlx::query_as(&sql).fetch_all(&mut conn).await?;
        let mut res = vec![];
        for row in rows.iter() {
            let entity = row.to_entity()?;
            res.push(entity)
        }
        Ok(res)
    }

    pub async fn get_students_by_codes(&self, codes: Vec<String>) -> Result<Vec<Student>, Error> {
        let sql = Query::select()
            .columns(TStudent::cols_all())
            .from(TStudent::Table)
            .and_where(
                Expr::col(TStudent::Code).is_in(codes),
            )
            .to_string(PostgresQueryBuilder);
        let mut conn = self.db.default.acquire().await?;
        let rows: Vec<RowTStudent> = sqlx::query_as(&sql).fetch_all(&mut conn).await?;
        let mut res = vec![];
        for row in rows.iter() {
            let entity = row.to_entity()?;
            res.push(entity)
        }
        Ok(res)
    }

    pub async fn get_students_by_filters(&self, req: GetStudentList) -> Result<(Vec<Student>, i64), Error> {
        let sql = Query::select()
            .columns(TStudent::cols_all())
            .from(TStudent::Table)
            .to_string(PostgresQueryBuilder);
        let mut conn = self.db.default.acquire().await?;
        let rows: Vec<RowTStudent> = sqlx::query_as(&sql).fetch_all(&mut conn).await?;
        let mut res = vec![];
        for row in rows.iter() {
            let entity = row.to_entity()?;
            res.push(entity)
        }

        let sql_count = Query::select()
            .expr(Func::count(Expr::col((TStudent::Table, TStudent::Id))))
            .from(TStudent::Table)
            .to_string(PostgresQueryBuilder);
        let count_row = sqlx::query(&sql_count).fetch_one(&mut conn).await?;
        let count = count_row.get(0);
        Ok((res, count))
    }

    pub async fn delete_student_by_id(&self, student_id: StudentId) -> Result<u64, Error> {
        let sql = Query::delete()
            .from_table(TStudent::Table)
            .and_where(
                Expr::col(TStudent::Id).eq(student_id)
            )
            .to_string(PostgresQueryBuilder);
        let mut conn = self.db.default.acquire().await?;
        let res = sqlx::query(&sql).execute(&mut conn).await?.rows_affected();
        Ok(res)
    }

    pub async fn enable_student_by_id(&self, student_id: StudentId) -> Result<u64, Error> {
        Err(NotImplemented)
    }

    pub async fn disable_student_by_id(&self, student_id: StudentId) -> Result<u64, Error> {
        Err(NotImplemented)
    }

    pub async fn add_student(&self, req: AddStudent) -> Result<Student, Error> {
        let student_id: StudentId = uuid7::uuid7().into();
        let now = Utc::now();
        let create_dt = NaiveDateTime::new(now.date_naive(), now.time());
        let create_by = Uuid::nil();
        let update_dt = NaiveDateTime::default();
        let update_by = Uuid::nil();
        let sql = Query::insert()
            .into_table(TStudent::Table)
            .columns(TStudent::cols_all())
            .values_panic([
                student_id.into(),
                req.code.clone().into(),
                req.name.clone().into(),
                (req.kind as i32).into(),
                create_dt.into(),
                create_by.into(),
                update_dt.into(),
                update_by.into(),
            ]).to_string(PostgresQueryBuilder);
        let mut conn = self.db.default.acquire().await?;
        let res = sqlx::query(&sql).execute(&mut conn).await?.rows_affected();
        if res <= 0 {
            return Err(Error::ServerError("创建失败".to_string()))
        }
        let s = Student {
            id: student_id,
            code: req.code,
            name: req.name,
            kind: req.kind,
            create_dt,
            create_by,
            update_dt,
            update_by,
        };
        Ok(s)
    }
}
