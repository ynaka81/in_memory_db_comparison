#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

use rand::Rng;
use std::sync::Arc;
use tonic::{Request, Response, Status};

// Use different RwLock implementations.
use async_std::sync::RwLock as AsyncStdRwLock;
use std::sync::RwLock as StdRwLock;
use tokio::sync::RwLock as TokioRwLock;

use db::{db_server::Db, Record, Records, Value};

pub mod db {
    tonic::include_proto!("db");
}

pub trait DbDesc {
    fn description(&self) -> String;
}

struct DbCommon;

impl DbCommon {
    fn initial_records() -> Vec<i32> {
        const N: i32 = 1e6 as i32;
        let mut rng = rand::thread_rng();
        (0..N).map(|_| rng.gen_range(0, N / 10)).collect()
    }

    #[inline]
    fn search<'a>(query: i32, records: impl Iterator<Item = &'a i32>) -> Records {
        let records = records
            .enumerate()
            .filter_map(|(i, &value)| {
                if value == query {
                    Some(Record {
                        index: i as i32,
                        value,
                    })
                } else {
                    None
                }
            })
            .collect();
        Records { records }
    }

    #[inline]
    fn update(updates: &[Record], records: &mut [i32]) {
        for record in updates {
            records[record.index as usize] = record.value;
        }
    }
}

// Define AsyncStdDb as an implementation of DB package.
// AsyncStdDb employs async_std::sync::RwLock.
#[derive(Debug)]
pub struct AsyncStdDb {
    records: Arc<AsyncStdRwLock<Vec<i32>>>,
}

impl Default for AsyncStdDb {
    fn default() -> Self {
        let records = DbCommon::initial_records();
        Self {
            records: Arc::new(AsyncStdRwLock::new(records)),
        }
    }
}

impl DbDesc for AsyncStdDb {
    fn description(&self) -> String {
        "AsyncStdDb (with async_std::sync::RwLock)".to_string()
    }
}

// Implement RPC methods on AsyncStdDb.
#[tonic::async_trait]
impl Db for AsyncStdDb {
    async fn search(&self, request: Request<Value>) -> Result<Response<Records>, Status> {
        let query = request.into_inner().value;
        let records = DbCommon::search(query, self.records.read().await.iter());

        Ok(Response::new(records))
    }

    async fn add(&self, request: Request<Value>) -> Result<Response<()>, Status> {
        let value = request.into_inner().value;
        self.records.write().await.push(value);

        Ok(Response::new(()))
    }

    async fn update(&self, request: Request<Records>) -> Result<Response<()>, Status> {
        DbCommon::update(
            &request.into_inner().records,
            &mut self.records.write().await,
        );
        Ok(Response::new(()))
    }
}

// Define StdDb as an implementation of DB package.
// StdDb employs std::sync::RwLock.
#[derive(Debug)]
pub struct StdDb {
    records: Arc<StdRwLock<Vec<i32>>>,
}

impl Default for StdDb {
    fn default() -> Self {
        let records = DbCommon::initial_records();
        Self {
            records: Arc::new(StdRwLock::new(records)),
        }
    }
}

impl DbDesc for StdDb {
    fn description(&self) -> String {
        "StdDb (with std::sync::RwLock)".to_string()
    }
}

// Implement RPC methods on StdDb.
#[tonic::async_trait]
impl Db for StdDb {
    async fn search(&self, request: Request<Value>) -> Result<Response<Records>, Status> {
        let query = request.into_inner().value;
        let records = DbCommon::search(query, self.records.read().unwrap().iter());

        Ok(Response::new(records))
    }

    async fn add(&self, request: Request<Value>) -> Result<Response<()>, Status> {
        let value = request.into_inner().value;
        self.records.write().unwrap().push(value);

        Ok(Response::new(()))
    }

    async fn update(&self, request: Request<Records>) -> Result<Response<()>, Status> {
        DbCommon::update(
            &request.into_inner().records,
            &mut self.records.write().unwrap(),
        );
        Ok(Response::new(()))
    }
}

// Define TokioDb as an implementation of DB package.
// TokioDb employs tokio::sync::RwLock.
#[derive(Debug)]
pub struct TokioDb {
    records: Arc<TokioRwLock<Vec<i32>>>,
}

impl Default for TokioDb {
    fn default() -> Self {
        let records = DbCommon::initial_records();
        Self {
            records: Arc::new(TokioRwLock::new(records)),
        }
    }
}

impl DbDesc for TokioDb {
    fn description(&self) -> String {
        "TokioDb (with tokio::sync::RwLock)".to_string()
    }
}

// Implement RPC methods on TokioDb.
#[tonic::async_trait]
impl Db for TokioDb {
    async fn search(&self, request: Request<Value>) -> Result<Response<Records>, Status> {
        let query = request.into_inner().value;
        let records = DbCommon::search(query, self.records.read().await.iter());

        Ok(Response::new(records))
    }

    async fn add(&self, request: Request<Value>) -> Result<Response<()>, Status> {
        let value = request.into_inner().value;
        self.records.write().await.push(value);

        Ok(Response::new(()))
    }

    async fn update(&self, request: Request<Records>) -> Result<Response<()>, Status> {
        DbCommon::update(
            &request.into_inner().records,
            &mut self.records.write().await,
        );
        Ok(Response::new(()))
    }
}
