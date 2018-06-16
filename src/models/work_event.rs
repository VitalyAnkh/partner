use diesel;
use diesel::prelude::*;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use chrono::{Local, NaiveDateTime};

use common::schema::work_event;

type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RawWorkEvent {
    pub id: i32,
    pub record_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkEvent {
    pub record_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime
}

impl CreateWorkEvent {

    pub fn into_work_event(&self, record_id: i32, date_time: &NaiveDateTime) -> WorkEvent {
      
      WorkEvent {
          record_id: record_id,
          start_time: self.start_time.clone(),
          end_time: self.end_time.clone(),
          create_time: date_time.clone(),
          update_time: date_time.clone() 
      }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="work_event"]
pub struct WorkEvent {
    pub record_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

impl WorkEvent {

    pub fn create(conn: &Conn, records: &Vec<WorkEvent>) -> QueryResult<usize> {
        use common::schema::work_event::dsl::*;

        diesel::insert_into(work_event).values(records).execute(conn)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEvents;

impl DeleteEvents {

    pub fn delete(&self, conn: &Conn, cur_record_id: i32) -> QueryResult<usize> {

        use common::schema::work_event::dsl::*;

        diesel::delete(work_event.filter(record_id.eq(cur_record_id))).execute(conn)
    }
}