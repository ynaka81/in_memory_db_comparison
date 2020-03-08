use std::sync::{Arc, RwLock};
use rand::Rng;
use tonic::{transport::Server, Request, Response, Status};

use db::db_server::{Db, DbServer};
use db::{Records, Record, Value};

pub mod db {
    tonic::include_proto!("db");
}

// Define DbImpl as implementation of DB package.
#[derive(Debug, Default)]
pub struct DbImpl {
    records: Arc<RwLock<Vec<i32>>>,
}

// Initialize with random records.
impl DbImpl {
    fn default() -> Self {
        const N: i32 = 1e6 as i32;
        let mut rng = rand::thread_rng();
        let records: Vec<i32> = (0..N).map(|_| rng.gen_range(0, N / 10)).collect();

        DbImpl {
            records: Arc::new(RwLock::new(records))
        }
    }
}

// Implement RPC methods.
#[tonic::async_trait]
impl Db for DbImpl {
    async fn search(&self, request: Request<Value>) -> Result<Response<Records>, Status> {
        let query = request.into_inner().value;
        let records = self.records.read().unwrap().iter().enumerate().filter_map(
            |(i, &value)|
            if value == query {
                Some(Record {
                    index: i as i32,
                    value: value
                })
            } else {
                None
            }).collect();
        let records = Records {
            records
        };

        Ok(Response::new(records))
    }

    async fn update(&self, request: Request<Records>) -> Result<Response<()>, Status> {
        let mut records = self.records.write().unwrap();
        for record in request.into_inner().records {
            records[record.index as usize] = record.value;
        };

        Ok(Response::new(()))
    }
}

// Run gRPC server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let db = DbImpl::default();

    Server::builder()
        .add_service(DbServer::new(db))
        .serve(addr)
        .await?;

    Ok(())
}
