use chrono::{NaiveDate, NaiveDateTime};
use deadpool_postgres::Client as PgClient;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Client;
use crate::infra::error::biz::BizKind::DataNotFound;
use crate::infra::error::error::ServiceError;
use crate::infra::error::error::Kind::BizError;

#[derive(Deserialize, PostgresMapper, Debug, Serialize, Default)]
#[pg_mapper(table = "Health")]
pub struct HealthRecord {
    pub id: i64,
    pub height: f64,
    pub weight: f64,
    pub teeth: i32,
    pub head_circumference: f64,
    pub record_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub(crate) async fn insert(pg_client: &PgClient, height: f64, weight: f64, teeth: i32, head_circumference: f64, record_date: NaiveDate) -> Result<HealthRecord, ServiceError> {
    let stmt = r#"
        INSERT INTO
            health (
                height,
                weight,
                teeth,
                head_circumference,
                record_date
            )
        VALUES
            ($1, $2, $3, $4, $5)
        RETURNING *;
    "#;

    let row = pg_client
        .query_one(stmt, &[&height, &weight, &teeth, &head_circumference, &record_date])
        .await?;

    let health_record = HealthRecord::from_row_ref(&row)?;

    Ok(health_record)
}

pub(crate) async fn select_all(client: &Client) -> Result<Vec<HealthRecord>, ServiceError> {
    let stmt = r#"
        SELECT
            *
        FROM
            health
        ORDER BY
            record_date DESC
    "#;

    let rows = client.query(stmt, &[]).await?;

    return if rows.is_empty() {
        Err(
            ServiceError::build()
                .belong(BizError(DataNotFound))
                .message("There is no health record")
                .done()
        )
    } else {
        let mut all_health_records = Vec::new();

        for row in rows {
            let health_record = HealthRecord::from_row_ref(&row)?;
            all_health_records.push(health_record)
        }

        Ok(all_health_records)
    }
}

pub(crate) async fn select_many(pc: &PgClient, page_number: i64, page_size: i64) -> Result<Vec<HealthRecord>, ServiceError> {
    debug!("page number: {}, page size: {}",page_number, page_size);

    let stmt = r#"
        SELECT
            *
        FROM
            health
        ORDER BY
            record_date DESC
        LIMIT
            $2
        OFFSET
            $1;
    "#;


    let offset = page_number * page_size;

    let rows = pc
        .query(stmt, &[&offset, &page_size])
        .await?;

    return if rows.is_empty() {
        Err(
            ServiceError::build()
                .belong(BizError(DataNotFound))
                .message("There is no health record while paginated querying")
                .done()
        )
    } else {
        let mut health = Vec::new();

        for row in rows {
            let health_record = HealthRecord::from_row_ref(&row)?;
            health.push(health_record)
        }

        Ok(health)
    };
}

pub(crate) async fn count(pc: &PgClient) -> Result<i64, ServiceError> {
    let stmt = r#"SELECT COUNT(*) FROM health"#;

    let count = pc.query_one(stmt, &[])
        .await?
        .get(0);

    Ok(count)
}