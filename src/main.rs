use tonic::transport::Server;
use tonic::Response;
use wamr_sdk::WamrSdkResponse;
use wamr_sdk::wamr_sdk_server::{WamrSdk,WamrSdkServer};

pub mod wamr_sdk { 
    include!("../stubs/wamr_sdk.rs");
}

#[derive(Debug,Default)]
pub struct MyWamrRuntime {

}

#[tonic::async_trait]
impl WamrSdk for MyWamrRuntime {
    async fn sdk_test(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<WamrSdkResponse>, tonic::Status> {
        println!("rpc function called successfully");
        let response = WamrSdkResponse {
            status:0,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() ->Result<(),Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080".parse()?;

    let wamr_sdk_service = MyWamrRuntime::default();

    println!("gRPC server is listening on: {}",addr);
    Server::builder().add_service(WamrSdkServer::new(wamr_sdk_service)).serve(addr).await.expect("failed to start gRPC server");

    Ok(())
}
