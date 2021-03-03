#[derive(Hash, Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    #[prost(string, tag = "1")]
    pub asset: ::prost::alloc::string::String,
}
#[derive(Hash, Clone, PartialEq, ::prost::Message)]
pub struct Dkg {
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
