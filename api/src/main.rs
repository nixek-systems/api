use tonic::{Request, Response, Status};
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

use anyhow::{Result, bail};
use protogen::*;
use protogen::bootstrap_api_server::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    listen_addr: String,

    server_cert: String,
    server_key: String,
    client_ca_cert: String,
}

#[derive(Debug, Default)]
pub struct Svc {}

#[tonic::async_trait]
impl BootstrapApi for Svc {
    async fn get_token_metadata(
        &self,
        _req: Request<BootstrapApiGetTokenMetadataReq>,
    ) -> Result<Response<BootstrapApiGetTokenMetadataResp>, Status> {
        unimplemented!()
    }

    async fn exchange_token(
        &self,
        _req: Request<BootstrapApiExchangeReq>,
    ) -> Result<Response<BootstrapApiExchangeResp>, Status> {
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

    let addr = conf.listen_addr.parse()?;
    let api = Svc::default();
    let client_ca_cert = Certificate::from_pem(conf.client_ca_cert);

    let server_identity = Identity::from_pem(conf.server_cert, conf.server_key);
    let tls = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert);

    Server::builder()
        .tls_config(tls)?
        .add_service(BootstrapApiServer::new(api))
        .serve(addr)
        .await?;

    Ok(())
}
