use clap::{arg_enum, value_t, App, Arg};
use std::net::SocketAddr;
use tokio::runtime;
use tonic::transport::Server;

use rust_server::{
    db::db_server::{Db, DbServer},
    AsyncStdDb, DbDesc, StdDb, TokioDb,
};

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum DbImpl {
        AsyncStd,
        Std,
        Tokio,
    }
}

// Run gRPC server.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("rust-server")
        .arg(
            Arg::with_name("parallel")
                .short("p")
                .long("parallel")
                .value_name("PARALLEL")
                .takes_value(true)
                .help("Number of the core threads for the async runtime. [default: 16]"),
        )
        .arg(
            Arg::with_name("DB-IMPL")
                .index(1)
                .required(true)
                .possible_values(&["tokio", "asyncstd", "std"])
                .case_insensitive(true)
                .help("DB implementation"),
        )
        .get_matches();

    let addr = "[::0]:50051".parse()?;
    let db_impl = value_t!(matches, "DB-IMPL", DbImpl).unwrap_or_else(|e| e.exit());
    let core_threads = matches
        .value_of("PARALLEL")
        .unwrap_or("16")
        .parse::<usize>()?;

    let mut rt = runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(core_threads)
        .enable_all()
        .build()?;

    println!(
        "Created the async runtime with {} core threads.",
        core_threads
    );

    match db_impl {
        DbImpl::AsyncStd => rt.block_on(run_server(addr, AsyncStdDb::default()))?,
        DbImpl::Std => rt.block_on(run_server(addr, StdDb::default()))?,
        DbImpl::Tokio => rt.block_on(run_server(addr, TokioDb::default()))?,
    }

    Ok(())
}

async fn run_server(addr: SocketAddr, db: impl Db + DbDesc) -> Result<(), tonic::transport::Error> {
    println!("Listening to {} using {}.", addr, db.description());
    Server::builder()
        .add_service(DbServer::new(db))
        .serve(addr)
        .await
}
