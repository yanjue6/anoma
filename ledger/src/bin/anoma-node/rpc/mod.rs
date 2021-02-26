use anoma::protobuf::gossip::gossip_service_server::{GossipService, GossipServiceServer};
use anoma::protobuf::gossip::{Dkg, Intent, Response};
use tonic::{Request as TonicRequest, Response as TonicResponse, Status};
use tonic::transport::Server;
use crate::gossip::network_behaviour::Behaviour;
use libp2p;

#[derive(Debug)]
struct RpcService {
    swarm: libp2p::Swarm<Behaviour>,
}

#[tonic::async_trait]
impl GossipService for RpcService {
    async fn send_intent(
        &self,
        request: TonicRequest<Intent>,
    ) -> Result<TonicResponse<Response>, Status> {
        let Intent { asset } = request.get_ref();

        // swarm
            // .gossipsub
            // .publish(Topic::new(orderbook::TOPIC), tix_bytes)

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
pub async fn rpc_server(swarm: libp2p::Swarm<Behaviour>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:39111".parse().unwrap();

    let rpc = RpcService { swarm };

    let svc = GossipServiceServer::new(rpc);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}