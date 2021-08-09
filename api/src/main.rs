use tonic::{Request, Response, Status};
use tonic::transport::Server;

use anyhow::{Result, bail};
use protogen::*;
use protogen::machine_api_server::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    postgres_uri: String,
    listen_addr: String,
}

#[derive(Debug)]
pub struct Svc {
    pgclient: tokio_postgres::Client,
}

#[tonic::async_trait]
impl MachineApi for Svc {
    async fn register(
        &self,
        _req: Request<MachineApiRegisterReq>,
    ) -> Result<Response<MachineApiRegisterResp>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("config file path must be provided as the only argument");
    }
    let conf: Config = serde_json::from_reader(std::fs::File::open(&args[1])?)?;

    let (pgclient, pgconn) = tokio_postgres::connect(&conf.postgres_uri, tokio_postgres::NoTls).await?;

    let addr = conf.listen_addr.parse()?;
    let api = Svc{
        pgclient,
    };

    tokio::spawn(async move {
        if let Err(e) = pgconn.await {
            eprintln!("connection error: {}", e);
        }
    });

    Server::builder()
        .add_service(MachineApiServer::new(api))
        .serve(addr)
        .await?;

    Ok(())
}
