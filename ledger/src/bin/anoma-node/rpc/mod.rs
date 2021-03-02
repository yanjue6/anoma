use anoma::protobuf::gossip::{Dkg, Intent};
use anoma::protobuf::service::gossip_service_server::{
    GossipService, GossipServiceServer,
};
use anoma::protobuf::service::Response;

use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request as TonicRequest, Response as TonicResponse, Status};

#[derive(Debug)]
struct RpcService {
    tx: mpsc::Sender<Intent>,
}

#[tonic::async_trait]
impl GossipService for RpcService {
    async fn send_intent(
        &self,
        request: TonicRequest<Intent>,
    ) -> Result<TonicResponse<Response>, Status> {
        let intent = request.get_ref();

        println!("received a intent {:?}", intent);

        self.tx.send(intent.clone()).await.unwrap();
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
pub async fn rpc_server(
    tx: mpsc::Sender<Intent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:39111".parse().unwrap();

    let rpc = RpcService { tx };

    let svc = GossipServiceServer::new(rpc);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
