use tonic::transport::Server;
use tonic::Response;
use wamr_sdk::wamr_sdk_server::{WamrSdk, WamrSdkServer};
use wamr_sdk::{
    WamrSdkDeleteResponse, WamrSdkKillResponse, WamrSdkNewInstanceResponse, WamrSdkStartResponse,
};

pub mod wamr_sdk {
    include!("../stubs/wamr_sdk.rs");
}

#[derive(Debug, Default)]
pub struct MyWamrRuntime {}

#[tonic::async_trait]
impl WamrSdk for MyWamrRuntime {
    async fn new_instance(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<WamrSdkNewInstanceResponse>, tonic::Status> {
        println!("new instance rpc function called sucessfully");
        let response = WamrSdkNewInstanceResponse { status: 0 };

        Ok(Response::new(response))
    }

    async fn start(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<WamrSdkStartResponse>, tonic::Status> {
        println!("start rpc function called successfully");

        let response = WamrSdkStartResponse { status: 0 };

        Ok(Response::new(response))
    }

    async fn kill(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<WamrSdkKillResponse>, tonic::Status> {
        println!("kill rpc function called successfully");
        let response = WamrSdkKillResponse { status: 0 };

        Ok(Response::new(response))
    }

    async fn delete(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<WamrSdkDeleteResponse>, tonic::Status> {
        println!("delete rpc function called successfully");

        let response = WamrSdkDeleteResponse { status: 0 };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080".parse()?;

    let wamr_sdk_service = MyWamrRuntime::default();

    println!("gRPC server is listening on: {}", addr);
    Server::builder()
        .add_service(WamrSdkServer::new(wamr_sdk_service))
        .serve(addr)
        .await
        .expect("failed to start gRPC server");

    Ok(())
}
