#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod gossip_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GossipServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GossipServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GossipServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(
            inner: T,
            interceptor: impl Into<tonic::Interceptor>,
        ) -> Self {
            let inner =
                tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn send_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::super::gossip::Intent>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service.GossipService/SendIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_dkg(
            &mut self,
            request: impl tonic::IntoRequest<super::super::gossip::Dkg>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/service.GossipService/SendDkg",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GossipServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GossipServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GossipServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gossip_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GossipServiceServer."]
    #[async_trait]
    pub trait GossipService: Send + Sync + 'static {
        async fn send_intent(
            &self,
            request: tonic::Request<super::super::gossip::Intent>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
        async fn send_dkg(
            &self,
            request: tonic::Request<super::super::gossip::Dkg>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GossipServiceServer<T: GossipService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GossipService> GossipServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(
            inner: T,
            interceptor: impl Into<tonic::Interceptor>,
        ) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for GossipServiceServer<T>
    where
        T: GossipService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/service.GossipService/SendIntent" => {
                    #[allow(non_camel_case_types)]
                    struct SendIntentSvc<T: GossipService>(pub Arc<T>);
                    impl<T: GossipService>
                        tonic::server::UnaryService<
                            super::super::gossip::Intent,
                        > for SendIntentSvc<T>
                    {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::gossip::Intent,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_intent(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendIntentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(
                                codec,
                                interceptor,
                            )
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.GossipService/SendDkg" => {
                    #[allow(non_camel_case_types)]
                    struct SendDkgSvc<T: GossipService>(pub Arc<T>);
                    impl<T: GossipService>
                        tonic::server::UnaryService<super::super::gossip::Dkg>
                        for SendDkgSvc<T>
                    {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::gossip::Dkg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).send_dkg(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendDkgSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(
                                codec,
                                interceptor,
                            )
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: GossipService> Clone for GossipServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GossipService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GossipService> tonic::transport::NamedService
        for GossipServiceServer<T>
    {
        const NAME: &'static str = "service.GossipService";
    }
}
