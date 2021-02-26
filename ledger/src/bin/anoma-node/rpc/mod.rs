use anoma::protobuf::gossip::gossip_service_server::{GossipService, GossipServiceServer};
use anoma::protobuf::gossip::{Dkg, Intent, Response};
use tonic::{Request as TonicRequest, Response as TonicResponse, Status};
use tonic::transport::Server;

#[derive(Debug)]
struct RpcService;

#[tonic::async_trait]
impl GossipService for RpcService {
    async fn send_intent(
        &self,
        request: TonicRequest<Intent>,
    ) -> Result<TonicResponse<Response>, Status> {
        let Intent { asset } = request.get_ref();
        println!("received a intent {}", asset);
        Ok(TonicResponse::new(Response::default()))
    }
    async fn send_dkg(
        &self,
        request: TonicRequest<Dkg>,
    ) -> Result<TonicResponse<Response>, Status> {
        let Dkg { msg } = request.get_ref();
        println!("received a intent {}", msg);
        Ok(TonicResponse::new(Response::default()))
    }
}

#[tokio::main]
async fn rpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:39111".parse().unwrap();

    let rpc = RpcService {};

    let svc = GossipServiceServer::new(rpc);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}